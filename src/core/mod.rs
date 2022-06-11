/// The core module.
///
/// This module contains core structures that are used across the system, and
/// provides a common way to handle system data.

use core::fmt;
use half::f16;
use rand::Rng;
use nanoid::nanoid;


/// the bytes trait is implemented by structures with a custom byte
/// represenation. Structures that implement this trait must also implement
/// `From<&[u8]>` which allows for the byte representation be to converted back
/// into the struct.
trait Bytes<'a>: From<&'a [u8]> { 
    fn to_bytes(&self) -> Vec<u8>;
}


/// Sensor Structure.
/// 
/// A structure representing sensor values.
#[derive(Debug, Clone, Copy)]
pub struct Sensor {
    /// The temperature value in °C.
    pub temperature: f16,
    /// The humidity value in %.
    pub humidity: f16,
}
impl Sensor {
    /// Creates a new sensor.
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        return Sensor {
            temperature: f16::from_f32(rng.gen_range(40.0..50.0)),
            humidity: f16::from_f32(rng.gen_range(10.0..20.0)),
        }
    }
    /// Updates the sensor with the latest values.
    pub fn update(&mut self) {
        let mut rng = rand::thread_rng();
        self.temperature = f16::from_f32(rng.gen_range(40.0..50.0));
        self.humidity = f16::from_f32(rng.gen_range(10.0..20.0));
    }
}
impl Bytes<'_> for Sensor {
    /// Returns the byte representation of the sensor. This byte representation
    /// is a vector of bytes of length `4`, such that the first 2 bytes
    /// correspond to the temperature as a 16-bit float in native endian bytes,
    /// and the last 2 bytes correspond to the humidity as a 16-bit float in 
    /// native endian bytes.
    fn to_bytes(&self) -> Vec<u8> {
        return [
            self.temperature.to_ne_bytes(),
            self.humidity.to_ne_bytes()
        ].concat();
    }
}
impl From<&[u8]> for Sensor {
    fn from(bytes: &[u8]) -> Self {
        return Sensor {
            temperature: f16::from_ne_bytes([bytes[0], bytes[1]]),
            humidity: f16::from_ne_bytes([bytes[2], bytes[3]]),
        }
    }
}
impl fmt::Display for Sensor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "temperature: {:.2} °C, humidity: {:.2}%", 
            self.temperature.to_f32(),
            self.humidity.to_f32()
        )
    }
}


/// Control System Structure.
/// 
/// This structure represents a controller system that handles and number of
/// sensors.
pub struct Controller {
    /// The id of the controller unit.
    pub id: String,
    /// The sensors connected to the unit.
    sensors: Vec<Sensor>,
}
impl Controller {
    /// Creates a new controller.
    pub fn new() -> Self {
        let mut sensors: Vec<Sensor> = Vec::new();
        for _ in 0..8 {
            sensors.push(Sensor::new());
        }
        return Controller { id: nanoid!(), sensors: sensors };
    }

    /// Returns the current readings of all sensors in byte format.
    /// 
    /// ### Format:
    /// The sensor data format looks like the following:
    /// |      | s_0     | s_1     | s_2     | s_3     | ... | s_n     |
    /// |------|---------|---------|---------|---------|-----|---------|
    /// | data | [u8; 4] | [u8; 4] | [u8; 4] | [u8; 4] | ... | [u8; 4] |
    pub fn sensor_data(&mut self) -> Vec<u8> {
        let data: Vec<Vec<u8>> = self.sensors.iter_mut().map(|sensor| {
            sensor.update();
            return sensor.to_bytes();
        }).collect();
        return data.concat();
    }
}
impl fmt::Display for Controller {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:\n", self.id)?;
        Ok(for (i, sensor) in self.sensors.iter().enumerate() {
            write!(f, "  zone {}: {}\n", i, sensor)?;
        })
    }
}
