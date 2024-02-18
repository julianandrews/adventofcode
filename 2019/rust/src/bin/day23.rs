#![feature(async_closure)]

use std::sync::Arc;

use anyhow::{anyhow, Result};
use futures::stream::{Stream, StreamExt};
use tokio::sync::{mpsc, Notify};

use aoc::intcode::{RegisterValue, VM};

type Address = usize;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let input = aoc::utils::get_input()?;
    let program: Vec<RegisterValue> = aoc::intcode::parse_program(input.trim())?;

    println!("Part 1: {}", part1(&program).await?);

    Ok(())
}

async fn part1(program: &[RegisterValue]) -> Result<RegisterValue> {
    let mut network = Network::start(program, 50);
    while let Some(message) = network.incoming.recv().await {
        println!("Processing {:?}", message);
        if message.destination == 255 {
            return Ok(message.data.y);
        }
        let channel = network
            .vm_channels
            .get(message.destination)
            .ok_or_else(|| anyhow!("Channel not found for {}.", message.destination))?;
        channel.send(message.data)?;
    }
    Err(anyhow!("Network terminated early"))
}

struct Network {
    vm_channels: Vec<NetworkedVMChannel>,
    incoming: mpsc::UnboundedReceiver<Message>,
}

impl Network {
    fn start(program: &[RegisterValue], number: Address) -> Network {
        let (outputs, incoming) = mpsc::unbounded_channel();
        let mut vm_channels = vec![];
        for address in 0..number {
            vm_channels.push(NetworkedVMChannel::spawn(program, address, outputs.clone()));
        }
        vm_channels[0].ready.notify_one();
        Network {
            vm_channels,
            incoming,
        }
    }
}

struct NetworkedVMChannel {
    channel: mpsc::UnboundedSender<Packet>,
    ready: Arc<Notify>,
}

impl NetworkedVMChannel {
    fn spawn(
        program: &[RegisterValue],
        address: Address,
        outputs: mpsc::UnboundedSender<Message>,
    ) -> NetworkedVMChannel {
        let (channel, inputs) = mpsc::unbounded_channel();
        let ready = Arc::new(Notify::new());
        let mut vm = NetworkedVM::new(program.to_vec(), address, inputs, outputs, ready.clone());
        tokio::spawn(async move { vm.run().await });
        NetworkedVMChannel { channel, ready }
    }

    fn send(&self, packet: Packet) -> Result<()> {
        self.channel.send(packet)?;
        self.ready.notify_one();
        Ok(())
    }
}

struct NetworkedVM<'a> {
    vm: VM<'a>,
    address: Address,
    channel: mpsc::UnboundedSender<Message>,
}

impl<'a> NetworkedVM<'a> {
    fn new(
        program: Vec<RegisterValue>,
        address: Address,
        inputs: mpsc::UnboundedReceiver<Packet>,
        channel: mpsc::UnboundedSender<Message>,
        ready: Arc<Notify>,
    ) -> NetworkedVM<'a> {
        let vm = VM::new(program.to_vec(), Box::pin(input_stream(inputs, ready)));
        NetworkedVM {
            vm,
            address,
            channel,
        }
    }

    async fn run(&mut self) -> Option<()> {
        let mut outputs = self.vm.outputs_async();
        loop {
            let destination = outputs.next().await?;
            let x = outputs.next().await?;
            let y = outputs.next().await?;
            let message = Message {
                source: self.address,
                destination: destination as usize, // TODO: handle error
                data: Packet { x, y },
            };
            println!("Sending {:?}", message);
            self.channel.send(message).unwrap(); // TODO
            tokio::task::yield_now().await; // TODO
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Message {
    source: Address,
    destination: Address,
    data: Packet,
}

#[derive(Debug, Clone, Copy)]
struct Packet {
    x: RegisterValue,
    y: RegisterValue,
}

fn input_stream(
    channel: mpsc::UnboundedReceiver<Packet>,
    ready: Arc<Notify>,
) -> impl Stream<Item = RegisterValue> {
    futures::stream::unfold(
        (channel, ready, None),
        async move |(mut channel, ready, next_value)| {
            if let Some(value) = next_value {
                return Some((value, (channel, ready, None)));
            }
            ready.notified().await;
            println!("Received Notification");
            match channel.try_recv() {
                Ok(message) => {
                    println!("Got {:?}", message);
                    Some((message.x, (channel, ready, Some(message.y))))
                }
                Err(mpsc::error::TryRecvError::Empty) => Some((-1, (channel, ready, None))),
                Err(mpsc::error::TryRecvError::Disconnected) => None,
            }
        },
    )
}
