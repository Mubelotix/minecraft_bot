#[path = "../src/nbt.rs"]
mod nbt;
use nbt::parse;

pub enum ContainerType {
    Normal,
    Container,
    Chest,
    CraftingTable,
    Furnace,
    Dispenser,
    EnchantingTable,
    BrewingStand,
    Villager,
    Beacon,
    Anvil,
    Hopper,
    Dropper,
    ShulkerBox,
    EntityHorse //TODO
}

pub struct InventoryManager {
    next_window: (ContainerType, usize),
    window: InventorySlots,
    inventory: InventorySlots,
    hotbar: InventorySlots
}

impl InventoryManager {
    pub fn new(data: &Vec<u8>) -> Self {
        let mut inventory = InventoryManager{
            next_window: (ContainerType::Normal, 0),
            window: InventorySlots::None,
            inventory: InventorySlots::None,
            hotbar: InventorySlots::None,
        };
        inventory.update(data).unwrap();
        inventory
    }

    pub fn new_empty() -> Self {
        let inventory = InventoryManager{
            next_window: (ContainerType::Normal, 0),
            window: InventorySlots::None,
            inventory: InventorySlots::None,
            hotbar: InventorySlots::None,
        };
        inventory
    }

    pub fn set_next_window(&mut self, window_type: ContainerType, window_capacity: usize) {
        self.next_window = (window_type, window_capacity);
        self.window = InventorySlots::None; // none because it needs to be updated
    }

    pub fn update(&mut self, data: &Vec<u8>) -> Result<(), &'static str> {
        let mut slots: Vec<Slot> = Vec::new();
        let mut i = 0;
        while i < data.len() {
            if data[i] == 0x01u8 {
                i += 1;
                let item_id = data[i];
                i += 1;
                let count = data[i];
                i += 1;
                let nbt_begin = i;
                if data[i] != 0x00u8 {
                    if data[i] != 0x0Au8 {
                        if data[i] != 1u8 {
                            //println!("strange 0 {}", i);
                        }
                        i += 1;
                    } else {
                        //println!("strange 1 {}", i);
                    }
                    let (value, incremented_i) = parse(&data, i+1, data[i], true);
                    i = incremented_i;
                    slots.push(Slot::new(item_id, count, data[nbt_begin..i].to_vec()))
                } else {
                    // No NBT data
                    i += 1;
                    slots.push(Slot::new(item_id, count, Vec::new()))
                }
            } else if data[i] == 0x00u8 {
                // Non present
                i += 1;
                slots.push(Slot::new(0u8, 0u8, Vec::new()))
            } else {
                i += 1;
                return Err("Reading has slipped !");
            }
        }
        println!("{} slots", slots.len());
        Ok(())
    }
}

enum InventorySlots {
    None,
    HotBar([Slot; 9]),
    MainInventory([Slot; 27]),
    Chest([Slot; 27]),
    LargeChest([Slot; 54]),
}

struct Slot {
    count: u8,
    item_id: u8,
    nbt_data: Vec<u8>
}

impl Slot {
    pub fn new(item_id: u8, count: u8, nbt: Vec<u8>) -> Self {
        Slot {
            count,
            item_id,
            nbt_data: nbt
        }
    }

    pub fn get_slot_data(&self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        if self.count == 0 {
            data.push(0u8);
        } else {
            data = self.nbt_data.clone();
            data.insert(0, self.item_id);
            data.insert(0, self.count);
            data.insert(0, 1u8);
        }
        data
    }
}