use super::*;

impl Default for DoctorLevelDB {
    fn default() -> Self {
        Self {
            exp: vec![
                500, 800, 1240, 1320, 1400, 1480, 1560, 1640, 1720, 1800, 1880, 1960, 2040, 2120, 2200, 2280, 2360, 2440, 2520,
                2600, 2680, 2760, 2840, 2920, 3000, 3080, 3160, 3240, 3350, 3460, 3570, 3680, 3790, 3900, 4200, 4500, 4800,
                5100, 5400, 5700, 6000, 6300, 6600, 6900, 7200, 7500, 7800, 8100, 8400, 8700, 9000, 9500, 10000, 10500, 11000,
                11500, 12000, 12500, 13000, 13500, 14000, 14500, 15000, 15500, 16000, 17000, 18000, 19000, 20000, 21000, 22000,
                23000, 24000, 25000, 26000, 27000, 28000, 29000, 30000, 31000, 32000, 33000, 34000, 35000, 36000, 37000, 38000,
                39000, 40000, 41000, 42000, 43000, 44000, 45000, 46000, 47000, 48000, 49000, 50000, 51000, 52000, 54000, 56000,
                58000, 60000, 62000, 64000, 66000, 68000, 70000, 73000, 76000, 79000, 82000, 85000, 88000, 91000, 94000, 97000,
                100000,
            ],
            ap: vec![
                82, 84, 86, 88, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110,
                111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 120, 120, 120, 120, 121, 121, 121, 121, 121, 122, 122, 122,
                122, 122, 123, 123, 123, 123, 123, 124, 124, 124, 124, 124, 125, 125, 125, 125, 125, 126, 126, 126, 126, 126,
                127, 127, 127, 127, 127, 128, 128, 128, 128, 128, 129, 129, 129, 129, 129, 130, 130, 130, 130, 130, 130, 130,
                130, 130, 130, 130, 130, 130, 130, 130, 130, 131, 131, 131, 131, 132, 132, 132, 132, 133, 133, 133, 133, 134,
                134, 134, 134, 135, 135, 135, 135,
            ],
        }
    }
}

impl DoctorLevelDB {
    /// 理智恢复速度(1 理智 = x 分钟)
    pub const ACTION_POINT_REGEN: usize = 6;
}