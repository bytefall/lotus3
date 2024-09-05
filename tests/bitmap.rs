use lotus3::data::Archive;
use lotus3::graphics::decode;

macro_rules! bitmap {
    ($($name:ident[$hash:expr]($key:expr, $a:expr, $b:expr);)*) => {
        $(
            #[test]
            fn $name() {
                let data = Archive::open(&lotus3::ARCHIVE_FILE_NAME).unwrap().get($key).unwrap();

                assert_eq!(
                    $hash,
                    format!("{:x}", md5::compute(&decode(data, $a, $b)))
                );
            }
        )*
    }
}

bitmap! {
    bitmap_decode_c00["ab2ddcf199b25bf16f8429b70724dacc"]("C00", 0, 224);
    bitmap_decode_c01["60dd9c511bb694eec8d580aaf512e0b6"]("C01", 0, 224);
    bitmap_decode_c02["ca328cd73eb1698a0478f5827974762f"]("C02", 0, 224);
    bitmap_decode_c03["b80d3b8777676fa41a9ade07a2762c25"]("C03", 0, 0);
    bitmap_decode_c04["f5539f421234f9ff2ea198a289d2414a"]("C04", 0, 0);
    bitmap_decode_c05["40208efdd9203a44dff471e3314810b7"]("C05", 0, 0);
    bitmap_decode_c06["4e650de8919ff91b49f8a98dae63abbf"]("C06", 0, 0);
    bitmap_decode_c07["13165e3417a11b451a836493c45b813a"]("C07", 0, 224);
    bitmap_decode_s70["eb53ae6b86479c5f757c0703f2a40326"]("S70", 255, 25);
    bitmap_decode_s80["5762bae1900a5307fbdb57ff6bdb1480"]("S80", 0, 240);
    bitmap_decode_s81["3066896f7f34beeeb2eb78961293cea6"]("S81", 0, 240);
    bitmap_decode_s82["c6200446811c4e244a257bb73fcb8ba8"]("S82", 0, 240);
    bitmap_decode_s83["52c00a6a95ca71bcb4ba7021829225ec"]("S83", 0, 240);
    bitmap_decode_s84["8ddbc362fc675ed0be58e3b26ac8f088"]("S84", 0, 240);
    bitmap_decode_s85["7099fcd0b9762835b5f3c7b0b8c2b42e"]("S85", 0, 240);
    bitmap_decode_s86["f3c1121ffd5f1c01661a039ccefe1bb8"]("S86", 0, 240);
    bitmap_decode_s88["787fd6803c2dfbac9602aa728fc7aadb"]("S88", 0, 240);
    bitmap_decode_s89["65c4a6a1c71a8a4a63f7f5f687d7125f"]("S89", 0, 240);
    bitmap_decode_s8a["b4ad4ff0b17635c2c66cc0f55ff126fd"]("S8A", 0, 240);
    bitmap_decode_s90["6f4f4a2c64eea1bde68aa39c28634adf"]("S90", 0, 240);
    bitmap_decode_s92["c28b80e049517f98cf700cf87db50ce9"]("S92", 0, 240);
    bitmap_decode_s96["1551f3afbd49a17dbf53f8d5f313b966"]("S96", 0, 240);
    bitmap_decode_s97["7d54fca539e30a667847dfd6ee4aecf3"]("S97", 0, 240);
    bitmap_decode_sa6["5c6dfacebeb09695e1a85f11dc83dbef"]("SA6", 0, 240);
    bitmap_decode_sb2["3e58f67745468e535c48903c6d19cbf3"]("SB2", 0, 240);
    bitmap_decode_sb5["e866349d48d0dd456f6092049f59a250"]("SB5", 0, 240);
    bitmap_decode_sb6["5f3d0510bb4a8b1577e50a1776a8ef7b"]("SB6", 0, 240);
    bitmap_decode_sb7["a80ac12b01adc579e8da639a342c6887"]("SB7", 0, 240);
    bitmap_decode_sb8["54493c79f0978b1b1d227c08e8d0398f"]("SB8", 0, 240);
    bitmap_decode_sb9["f84421e2bfb45ff5e868523fecf27899"]("SB9", 0, 240);
    bitmap_decode_sba["70a7014d48ba9602f39f33966e685543"]("SBA", 0, 240);
    bitmap_decode_sbb["aa2de6565c56014aa33194a502e78cdf"]("SBB", 0, 240);
    bitmap_decode_sbc["c2756caf8ac59468899d7dfa41bbd298"]("SBC", 0, 240);
    bitmap_decode_sbd["e3b7d697e6ad11f8261c06fa80834557"]("SBD", 0, 240);
    bitmap_decode_sbe["ff645af987ee59f1d783ff4fd5c7b237"]("SBE", 255, 27);
    bitmap_decode_sc0["5c9fd727fae614bef21025b490a87e76"]("SC0", 0, 240);
    bitmap_decode_sc1["6f6edfafda28e31f9c281899590a45f9"]("SC1", 0, 240);
    bitmap_decode_sc2["ef813156d5b181e3385c0588364753cd"]("SC2", 0, 240);
    bitmap_decode_sc6["cb5f74848612b06e1407bb75ee0d6d3b"]("SC6", 0, 240);
    bitmap_decode_sc7["6be964e902c5e00d4c2d1fd84462efd1"]("SC7", 0, 240);
    bitmap_decode_sc8["da6d548d667c3e54b957c43c364797ab"]("SC8", 0, 240);
    bitmap_decode_sd1["242a72c554090b28a73c04c469688759"]("SD1", 0, 240);
    bitmap_decode_sd2["76f49b9898b6ff4726e90d9913e4b51b"]("SD2", 0, 240);
    bitmap_decode_sd3["6ef1b6244d3281ee01ffc9a5faa8ad87"]("SD3", 0, 240);
    bitmap_decode_sd4["3f558f04f1498031803e314681dcbe11"]("SD4", 0, 240);
    bitmap_decode_sd5["1b16a4058b231b4ff0999c0d9b4836c3"]("SD5", 0, 240);
    bitmap_decode_sd6["38952b31751ce836dcb93b5a952b2013"]("SD6", 0, 240);
    bitmap_decode_sd7["039ec31cbe147625709c5cb0bcd63aee"]("SD7", 0, 240);
    bitmap_decode_sd8["d2668f382d4239afa3df53ee05d9dc9d"]("SD8", 0, 240);
    bitmap_decode_sd9["940aa37e393e916232346de378fbd1b0"]("SD9", 0, 240);
    bitmap_decode_sda["464008f89b152f3a5266e97899308d12"]("SDA", 0, 240);
    bitmap_decode_sdb["6e8d6233fe494ef68e5a012f9a172704"]("SDB", 0, 240);
    bitmap_decode_sdc["069ad741da3efba120ba9ff6df0c5905"]("SDC", 0, 240);
    bitmap_decode_sdd["0074fffb98c96049990f31d2fc70286f"]("SDD", 0, 240);
    bitmap_decode_sde["82899b244097d0cee4d83a4fd3a62599"]("SDE", 0, 240);
    bitmap_decode_sdf["e06b17acf5e576e83ee439d1887967c2"]("SDF", 0, 240);
    bitmap_decode_se2["4ebf669e2900f4a6136f1a1a62f3f985"]("SE2", 0, 240);
    bitmap_decode_se3["e91597415a81c7751446dc863cb40ade"]("SE3", 0, 240);
    bitmap_decode_se4["25dfbff635530f5b1c9af15b8d8232f9"]("SE4", 0, 240);
    bitmap_decode_se5["e758cab649b292a065ed1b35770ea404"]("SE5", 0, 240);
    bitmap_decode_se7["00495a75fae59cfaeed5bbe1bc99a529"]("SE7", 0, 240);
    bitmap_decode_se9["5cf9a5587c0a757ead4af57106e0a51b"]("SE9", 0, 240);
    bitmap_decode_sea["7673f3a4b742e934446830688298af99"]("SEA", 0, 240);
    bitmap_decode_seb["66efac8084ab40aaec0e2c28e049c825"]("SEB", 0, 240);
    bitmap_decode_sec["56d2b6894c0bdcb8869e8b694b616824"]("SEC", 0, 240);
    bitmap_decode_sed["b15accbbfb0a07f69248cfce308911fb"]("SED", 0, 240);
}
