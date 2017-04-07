pub mod debugger;

use self::debugger::Debugger;
use Cartridge;
use Cpu;
use Memory;

pub struct GameBoy<'a> {
    pub cpu: &'a mut Cpu,
    pub cartridge: &'a Cartridge,
    pub mem: &'a mut Memory,
    pub debugger: Option<Debugger>,
}


impl<'a> GameBoy<'a> {
    pub fn new(cpu: &'a mut Cpu, cartridge: &'a Cartridge, mem: &'a mut Memory) -> GameBoy<'a> {
        GameBoy {
            cpu: cpu,
            cartridge: cartridge,
            mem: mem,
            debugger: None,
        }
    }

    pub fn enable_debug(&mut self) {
        self.debugger = Debugger::new();
    }

    pub fn event(&mut self) -> bool {
        match self.debugger {
            Some(ref mut d) => {
                match d.step {
                    0 => false,
                    _ => {
                        d.step -= 1;
                        true
                    }
                }
            }
            _ => true,
        }
    }
}
