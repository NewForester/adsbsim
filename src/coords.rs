//! ADS-B Simulator - see README.md
//
// © NewForester, 2018.  Available under MIT licence terms.
//
//! The coords module stores the position and velocity of an aerial vehicle
//! and provides a large number of getter and setter functions to handle
//! conversion between internal and external co-ordinate systems safely.
//!
//! The internal representation is m and m/s so that `update_position()` is a
//! simple addition (the ADS-B Simulator generates new positions once a second).
//!
//! Many of the getter/setter functions are provided for the convenience of
//! `set_candv()` and `get_canv()` functions of MAVLink message modules.
//!
//! The `set_cli()` function provides the implementation of the parsing of
//! certain command line parameters as described in README.md.
//!
//! The functions that do the real work are the setters:
//!
//!   * `set_position()`
//!   * `set_velocity()`
//!   * `set_course()`
//!   * `update_position()`
//!
use std::str::FromStr;

use std::f32;

// ---------------------------------------------------------------------

/*
 * Position and velocity of a UAV or UFO
 */

#[derive(Clone)]
pub struct CwithV {
    latitude:       f32,
    longitude:      f32,
    altitude:       f32,

    ud_velocity:    f32,
    ns_velocity:    f32,
    ew_velocity:    f32,
}

impl CwithV {
    pub fn new() -> CwithV {
        CwithV {
            latitude:       0.0,
            longitude:      0.0,
            altitude:       0.0,

            ud_velocity:    0.0,
            ns_velocity:    0.0,
            ew_velocity:    0.0,
        }
    }

    fn drop_end_characters(string: &str) ->  &str {
        &string[1 .. string.len() - 1]
    }

    fn drop_final_character(string: &str) ->  &str {
        &string[.. string.len() - 1]
    }

    fn convert_or(string: &str, default: f32) -> f32 {
        f32::from_str(string).unwrap_or(default)
    }

    const HOME_LAT: f32 = 51.10116770;
    const HOME_LONG: f32 = -2.05134590;
    const HOME_ALT: f32 = 128.0;

    pub fn set_cli(&mut self, cli: &str) -> &mut Self {
        let fission: Vec<&str> = cli.split(',').collect();

        let coords: Vec<&str> = Self::drop_end_characters(fission[0]).split(" ").collect();

        if coords[0].ends_with("m") {
            let metres = Self::drop_final_character(coords[0]);

            self.latitude = Self::get_latitude_m(Self::HOME_LAT) + Self::convert_or(metres, 0.0);
        }
        else
        {
            self.latitude = Self::get_latitude_m(Self::convert_or(coords[0], Self::HOME_LAT));
        }

        if coords[1].ends_with("m") {
            let metres = Self::drop_final_character(coords[1]);

            self.longitude = Self::get_longitude_m(Self::HOME_LONG) + Self::convert_or(metres, 0.0);
        }
        else
        {
            self.longitude = Self::get_longitude_m(Self::convert_or(coords[1], Self::HOME_LONG));
        }

        if coords[2].ends_with("m") {
            let metres = Self::drop_final_character(coords[2]);

            self.altitude = Self::HOME_ALT + Self::convert_or(metres, 0.0);
        }
        else
        {
            self.altitude = Self::convert_or(coords[2], Self::HOME_ALT);
        }

        let vels: Vec<&str> = Self::drop_end_characters(fission[1]).split(" ").collect();

        self.ns_velocity = Self::convert_or(vels[0], 0.0);
        self.ew_velocity = Self::convert_or(vels[1], 0.0);
        self.ud_velocity = Self::convert_or(vels[2], 0.0);

        self
    }

    pub fn set_position(&mut self, x: f32, y: f32, z: f32) -> &mut Self {
        self.latitude   = Self::get_latitude_m(x);
        self.longitude  = Self::get_longitude_m(y);
        self.altitude   = z;

        self
    }

    pub fn set_velocity(&mut self, vx: f32, vy: f32, vz: f32) -> &mut Self {
        self.ns_velocity = vx;
        self.ew_velocity = vy;
        self.ud_velocity = vz;

        self
    }

    pub fn set_course(&mut self, new: &CwithV) -> &mut Self {
        self.ns_velocity = new.ns_velocity;
        self.ew_velocity = new.ew_velocity;
        self.ud_velocity = new.ud_velocity;

        self
    }

    pub fn update_position(&mut self) -> &mut Self {
        self.latitude  += self.ns_velocity;
        self.longitude += self.ew_velocity;
        self.altitude  += self.ud_velocity;

        self
    }

    pub fn get_latitude(&self) -> f32 {
        Self::get_latitude_degrees(self.latitude)
    }
    pub fn get_longitude(&self) -> f32 {
        Self::get_longitude_degrees(self.longitude)
    }
    pub fn get_altitude(&self) -> f32 {
        self.altitude
    }

    pub fn get_rateofclimb(&self) -> f32 {
        self.ud_velocity
    }

    pub fn get_ns_velocity(&self) -> f32 {
        self.ns_velocity
    }
    pub fn get_ew_velocity(&self) -> f32 {
        self.ew_velocity
    }

    pub fn get_heading(&self) -> f32 {
        let mut heading: f64;

        let ew_velocity = self.ew_velocity as f64;
        let ns_velocity = self.ns_velocity as f64;

        heading = if ew_velocity == 0.0 {
            0.0
        } else {
            90.0 - (ns_velocity/ew_velocity).abs().atan().to_degrees()
        };

        if ns_velocity < 0.0 {
            heading = 180.0 - heading;
        }

        if ew_velocity < 0.0 {
            heading = 360.0 - heading;
        }

        heading as f32
    }
    pub fn get_groundspeed(&self) -> f32 {
        let ew_velocity = self.ew_velocity as f64;
        let ns_velocity = self.ns_velocity as f64;

        (ew_velocity * ew_velocity + ns_velocity * ns_velocity).sqrt() as f32
    }

    const DEGREE_TO_M_LAT: f32 = 111229.0639884;
    const DEGREE_TO_M_LONG: f32 = 71695.7536163;

    fn get_latitude_m(latitude_degrees: f32) -> f32 {
        latitude_degrees * Self::DEGREE_TO_M_LAT
    }
    fn get_longitude_m(longitude_degrees: f32) -> f32 {
        longitude_degrees * Self::DEGREE_TO_M_LONG
    }

    fn get_latitude_degrees(latitude_m: f32) -> f32 {
        latitude_m / Self::DEGREE_TO_M_LAT
    }
    fn get_longitude_degrees(longitude_m: f32) -> f32 {
        longitude_m / Self::DEGREE_TO_M_LONG
    }
}

// EOF
