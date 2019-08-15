use std::{thread, time};
use std::sync::mpsc::{channel, Sender};
use std::io;
use ozelot::{Client, serverbound, utils};
use ozelot::clientbound::*;
mod inventory;
use inventory::*;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut client = Client::connect_unauthenticated("playmc.games", 25565, "mubelotix").unwrap();
    let mut inventory = InventoryManager::new_empty();

    let (tx, rx) = channel();
    thread::spawn(move || {
        read_stdin(tx);
    });

    'main: loop {
        let packets = client.read().unwrap();
        for packet in packets {
            match packet {
                ClientboundPacket::PlayDisconnect(ref p) => {
                    println!("Got disconnect packet, exiting ...");
                    println!("Reason: {}", utils::chat_to_str(p.get_reason()).unwrap());
                    break 'main;
                },
                ClientboundPacket::ChatMessage(ref p) => {
                    let msg = utils::chat_to_str(p.get_chat()).unwrap();
                    println!("{}", msg);
                },
                ClientboundPacket::OpenWindow(ref p) => {
                    let window_type = match p.get_window_type().as_str() {
                        "container" => ContainerType::Container,
                        "chest" => ContainerType::Chest,
                        "crafting_table" => ContainerType::CraftingTable,
                        "furnace" => ContainerType::Furnace,
                        "dispenser" => ContainerType::Dispenser,
                        "enchanting_table" => ContainerType::EnchantingTable,
                        "brewing_stand" => ContainerType::BrewingStand,
                        "villager" => ContainerType::Villager,
                        "beacon" => ContainerType::Beacon,
                        "anvil" => ContainerType::Anvil,
                        "hopper" => ContainerType::Hopper,
                        "dropper" => ContainerType::Dropper,
                        "shulker_box" => ContainerType::ShulkerBox,
                        "EntityHorse" => ContainerType::EntityHorse,
                        _ => ContainerType::Container,
                    };
                },
                ClientboundPacket::WindowItems(ref p) => {
                    let mut data = p.get_slots().clone();
                    data.remove(0);
                    data.remove(0);

                    let mut buffer = File::create("maj.txt").unwrap();
                    buffer.write(&data);
                    inventory.update(&data).unwrap();
                },
                _ => (),
            }
        }

        if let Ok(msg) = rx.try_recv() {
            let msg = msg.trim_end().to_string();
            if msg != String::from("pick") {
                println!("{}", msg);
                let chat = serverbound::ChatMessage::new(msg);
                client.send(chat).unwrap();
            } else {
                let request = serverbound::HeldItemChange::new(4);
                client.send(request).unwrap();
                let request = serverbound::UseItem::new(0);
                client.send(request).unwrap();
                //let request = serverbound::ClickWindow::new(1, 10, 0, 0);
                //client.send(request).unwrap();
            }
            
        }

        thread::sleep(time::Duration::from_millis(50));
    }
}

fn read_stdin(tx: Sender<String>) {
    loop {
        let mut tmp = String::new();
        let _: usize = io::stdin().read_line(&mut tmp).unwrap();
        tx.send(tmp).unwrap();
    }
}
