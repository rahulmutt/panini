mod common;

use common::{CELLS, LAKARA_BY_NAME};
use panini::{Panini, Verdict};
use panini_data::{Lakara, Pada, Purusha, Vacana, dhatus};
use panini_prakriya::derive;

/// (root_number, lakara_label, pada, [P.E, P.D, P.B, M.E, M.D, M.B, U.E, U.D, U.B])
/// in SLP1. `PARADIGM`'s first column is a `Dhatu::dhatupatha` — the entry
/// number, unique by construction, so the two √aś rows are distinct without
/// anyone deciding which gaṇa's was the incumbent (`09.0059` kryādi,
/// `05.0020` svādi). Resolve a number against the `DHATUS` table in
/// `panini-data` to see which root a block is for; the tables carry no
/// per-row comment, deliberately, since 450 uncheckable comments is a
/// staleness liability no test could pin. The pada column is no longer
/// inferable from the root alone: 1.3.72 gives some roots a
/// `PadaAssignment` that admits both, so a block has to declare which pada
/// it is a block OF.
const PARADIGM: &[(&str, &str, Pada, [&str; 9])] = &[
    (
        "01.0001",
        "laT",
        Pada::Parasmaipada,
        [
            "Bavati", "BavataH", "Bavanti", "Bavasi", "BavaTaH", "BavaTa", "BavAmi", "BavAvaH",
            "BavAmaH",
        ],
    ),
    (
        "01.1049",
        "laT",
        Pada::Parasmaipada,
        [
            "nayati", "nayataH", "nayanti", "nayasi", "nayaTaH", "nayaTa", "nayAmi", "nayAvaH",
            "nayAmaH",
        ],
    ),
    (
        "01.1049",
        "laT",
        Pada::Atmanepada,
        [
            "nayate", "nayete", "nayante", "nayase", "nayeTe", "nayaDve", "naye", "nayAvahe",
            "nayAmahe",
        ],
    ),
    (
        "01.0642",
        "laT",
        Pada::Parasmaipada,
        [
            "jayati", "jayataH", "jayanti", "jayasi", "jayaTaH", "jayaTa", "jayAmi", "jayAvaH",
            "jayAmaH",
        ],
    ),
    (
        "01.1082",
        "laT",
        Pada::Parasmaipada,
        [
            "smarati", "smarataH", "smaranti", "smarasi", "smaraTaH", "smaraTa", "smarAmi",
            "smarAvaH", "smarAmaH",
        ],
    ),
    (
        "01.0381",
        "laT",
        Pada::Parasmaipada,
        [
            "paWati", "paWataH", "paWanti", "paWasi", "paWaTaH", "paWaTa", "paWAmi", "paWAvaH",
            "paWAmaH",
        ],
    ),
    (
        "01.1164",
        "laT",
        Pada::Parasmaipada,
        [
            "vadati", "vadataH", "vadanti", "vadasi", "vadaTaH", "vadaTa", "vadAmi", "vadAvaH",
            "vadAmaH",
        ],
    ),
    (
        "01.0001",
        "laN",
        Pada::Parasmaipada,
        [
            "aBavad", "aBavatAm", "aBavan", "aBavaH", "aBavatam", "aBavata", "aBavam", "aBavAva",
            "aBavAma",
        ],
    ),
    (
        "01.1049",
        "laN",
        Pada::Parasmaipada,
        [
            "anayad", "anayatAm", "anayan", "anayaH", "anayatam", "anayata", "anayam", "anayAva",
            "anayAma",
        ],
    ),
    (
        "01.1049",
        "laN",
        Pada::Atmanepada,
        [
            "anayata",
            "anayetAm",
            "anayanta",
            "anayaTAH",
            "anayeTAm",
            "anayaDvam",
            "anaye",
            "anayAvahi",
            "anayAmahi",
        ],
    ),
    (
        "01.0642",
        "laN",
        Pada::Parasmaipada,
        [
            "ajayad", "ajayatAm", "ajayan", "ajayaH", "ajayatam", "ajayata", "ajayam", "ajayAva",
            "ajayAma",
        ],
    ),
    (
        "01.1082",
        "laN",
        Pada::Parasmaipada,
        [
            "asmarad",
            "asmaratAm",
            "asmaran",
            "asmaraH",
            "asmaratam",
            "asmarata",
            "asmaram",
            "asmarAva",
            "asmarAma",
        ],
    ),
    (
        "01.0381",
        "laN",
        Pada::Parasmaipada,
        [
            "apaWad", "apaWatAm", "apaWan", "apaWaH", "apaWatam", "apaWata", "apaWam", "apaWAva",
            "apaWAma",
        ],
    ),
    (
        "01.1164",
        "laN",
        Pada::Parasmaipada,
        [
            "avadad", "avadatAm", "avadan", "avadaH", "avadatam", "avadata", "avadam", "avadAva",
            "avadAma",
        ],
    ),
    (
        "01.0001",
        "loT",
        Pada::Parasmaipada,
        [
            "Bavatu", "BavatAm", "Bavantu", "Bava", "Bavatam", "Bavata", "BavAni", "BavAva",
            "BavAma",
        ],
    ),
    (
        "01.1049",
        "loT",
        Pada::Parasmaipada,
        [
            "nayatu", "nayatAm", "nayantu", "naya", "nayatam", "nayata", "nayAni", "nayAva",
            "nayAma",
        ],
    ),
    (
        "01.1049",
        "loT",
        Pada::Atmanepada,
        [
            "nayatAm", "nayetAm", "nayantAm", "nayasva", "nayeTAm", "nayaDvam", "nayE", "nayAvahE",
            "nayAmahE",
        ],
    ),
    (
        "01.0642",
        "loT",
        Pada::Parasmaipada,
        [
            "jayatu", "jayatAm", "jayantu", "jaya", "jayatam", "jayata", "jayAni", "jayAva",
            "jayAma",
        ],
    ),
    (
        "01.1082",
        "loT",
        Pada::Parasmaipada,
        [
            // uttama eka is smarARi, not smarAni: smar's r and the Ani
            // ending's n are separated only by the aw vowel A (6.1.101's
            // savarRa merge), so 8.4.2 retroflexes it -- same mechanism as
            // karavARi (< kf). Contrast BavAni (< BU), which has no r and
            // keeps its dental n. This entry predates Ratva modelling and
            // was wrong; the new tripAdI rules 8.4.1/8.4.2 caught it.
            "smaratu", "smaratAm", "smarantu", "smara", "smaratam", "smarata", "smarARi", "smarAva",
            "smarAma",
        ],
    ),
    (
        "01.0381",
        "loT",
        Pada::Parasmaipada,
        [
            "paWatu", "paWatAm", "paWantu", "paWa", "paWatam", "paWata", "paWAni", "paWAva",
            "paWAma",
        ],
    ),
    (
        "01.1164",
        "loT",
        Pada::Parasmaipada,
        [
            "vadatu", "vadatAm", "vadantu", "vada", "vadatam", "vadata", "vadAni", "vadAva",
            "vadAma",
        ],
    ),
    (
        "01.0001",
        "viDiliN",
        Pada::Parasmaipada,
        [
            "Baved", "BavetAm", "BaveyuH", "BaveH", "Bavetam", "Baveta", "Baveyam", "Baveva",
            "Bavema",
        ],
    ),
    (
        "01.1049",
        "viDiliN",
        Pada::Parasmaipada,
        [
            "nayed", "nayetAm", "nayeyuH", "nayeH", "nayetam", "nayeta", "nayeyam", "nayeva",
            "nayema",
        ],
    ),
    (
        "01.1049",
        "viDiliN",
        Pada::Atmanepada,
        [
            "nayeta",
            "nayeyAtAm",
            "nayeran",
            "nayeTAH",
            "nayeyATAm",
            "nayeDvam",
            "nayeya",
            "nayevahi",
            "nayemahi",
        ],
    ),
    (
        "01.0642",
        "viDiliN",
        Pada::Parasmaipada,
        [
            "jayed", "jayetAm", "jayeyuH", "jayeH", "jayetam", "jayeta", "jayeyam", "jayeva",
            "jayema",
        ],
    ),
    (
        "01.1082",
        "viDiliN",
        Pada::Parasmaipada,
        [
            "smared", "smaretAm", "smareyuH", "smareH", "smaretam", "smareta", "smareyam",
            "smareva", "smarema",
        ],
    ),
    (
        "01.0381",
        "viDiliN",
        Pada::Parasmaipada,
        [
            "paWed", "paWetAm", "paWeyuH", "paWeH", "paWetam", "paWeta", "paWeyam", "paWeva",
            "paWema",
        ],
    ),
    (
        "01.1164",
        "viDiliN",
        Pada::Parasmaipada,
        [
            "vaded", "vadetAm", "vadeyuH", "vadeH", "vadetam", "vadeta", "vadeyam", "vadeva",
            "vadema",
        ],
    ),
    (
        "01.0002",
        "laT",
        Pada::Atmanepada,
        [
            "eDate", "eDete", "eDante", "eDase", "eDeTe", "eDaDve", "eDe", "eDAvahe", "eDAmahe",
        ],
    ),
    (
        "01.1130",
        "laT",
        Pada::Atmanepada,
        [
            "laBate", "laBete", "laBante", "laBase", "laBeTe", "laBaDve", "laBe", "laBAvahe",
            "laBAmahe",
        ],
    ),
    (
        "01.0574",
        "laT",
        Pada::Atmanepada,
        [
            "sevate", "sevete", "sevante", "sevase", "seveTe", "sevaDve", "seve", "sevAvahe",
            "sevAmahe",
        ],
    ),
    (
        "01.0862",
        "laT",
        Pada::Atmanepada,
        [
            "vartate",
            "vartete",
            "vartante",
            "vartase",
            "varteTe",
            "vartaDve",
            "varte",
            "vartAvahe",
            "vartAmahe",
        ],
    ),
    (
        "01.0696",
        "laT",
        Pada::Atmanepada,
        [
            "BAzate", "BAzete", "BAzante", "BAzase", "BAzeTe", "BAzaDve", "BAze", "BAzAvahe",
            "BAzAmahe",
        ],
    ),
    (
        "01.0694",
        "laT",
        Pada::Atmanepada,
        [
            "Ikzate", "Ikzete", "Ikzante", "Ikzase", "IkzeTe", "IkzaDve", "Ikze", "IkzAvahe",
            "IkzAmahe",
        ],
    ),
    (
        "01.0002",
        "loT",
        Pada::Atmanepada,
        [
            "eDatAm", "eDetAm", "eDantAm", "eDasva", "eDeTAm", "eDaDvam", "eDE", "eDAvahE",
            "eDAmahE",
        ],
    ),
    (
        "01.1130",
        "loT",
        Pada::Atmanepada,
        [
            "laBatAm", "laBetAm", "laBantAm", "laBasva", "laBeTAm", "laBaDvam", "laBE", "laBAvahE",
            "laBAmahE",
        ],
    ),
    (
        "01.0574",
        "loT",
        Pada::Atmanepada,
        [
            "sevatAm", "sevetAm", "sevantAm", "sevasva", "seveTAm", "sevaDvam", "sevE", "sevAvahE",
            "sevAmahE",
        ],
    ),
    (
        "01.0862",
        "loT",
        Pada::Atmanepada,
        [
            "vartatAm",
            "vartetAm",
            "vartantAm",
            "vartasva",
            "varteTAm",
            "vartaDvam",
            "vartE",
            "vartAvahE",
            "vartAmahE",
        ],
    ),
    (
        "01.0696",
        "loT",
        Pada::Atmanepada,
        [
            "BAzatAm", "BAzetAm", "BAzantAm", "BAzasva", "BAzeTAm", "BAzaDvam", "BAzE", "BAzAvahE",
            "BAzAmahE",
        ],
    ),
    (
        "01.0694",
        "loT",
        Pada::Atmanepada,
        [
            "IkzatAm", "IkzetAm", "IkzantAm", "Ikzasva", "IkzeTAm", "IkzaDvam", "IkzE", "IkzAvahE",
            "IkzAmahE",
        ],
    ),
    (
        "01.0002",
        "laN",
        Pada::Atmanepada,
        [
            "EData", "EDetAm", "EDanta", "EDaTAH", "EDeTAm", "EDaDvam", "EDe", "EDAvahi", "EDAmahi",
        ],
    ),
    (
        "01.1130",
        "laN",
        Pada::Atmanepada,
        [
            "alaBata",
            "alaBetAm",
            "alaBanta",
            "alaBaTAH",
            "alaBeTAm",
            "alaBaDvam",
            "alaBe",
            "alaBAvahi",
            "alaBAmahi",
        ],
    ),
    (
        "01.0574",
        "laN",
        Pada::Atmanepada,
        [
            "asevata",
            "asevetAm",
            "asevanta",
            "asevaTAH",
            "aseveTAm",
            "asevaDvam",
            "aseve",
            "asevAvahi",
            "asevAmahi",
        ],
    ),
    (
        "01.0862",
        "laN",
        Pada::Atmanepada,
        [
            "avartata",
            "avartetAm",
            "avartanta",
            "avartaTAH",
            "avarteTAm",
            "avartaDvam",
            "avarte",
            "avartAvahi",
            "avartAmahi",
        ],
    ),
    (
        "01.0696",
        "laN",
        Pada::Atmanepada,
        [
            "aBAzata",
            "aBAzetAm",
            "aBAzanta",
            "aBAzaTAH",
            "aBAzeTAm",
            "aBAzaDvam",
            "aBAze",
            "aBAzAvahi",
            "aBAzAmahi",
        ],
    ),
    (
        "01.0694",
        "laN",
        Pada::Atmanepada,
        [
            "Ekzata", "EkzetAm", "Ekzanta", "EkzaTAH", "EkzeTAm", "EkzaDvam", "Ekze", "EkzAvahi",
            "EkzAmahi",
        ],
    ),
    (
        "01.0002",
        "viDiliN",
        Pada::Atmanepada,
        [
            "eDeta", "eDeyAtAm", "eDeran", "eDeTAH", "eDeyATAm", "eDeDvam", "eDeya", "eDevahi",
            "eDemahi",
        ],
    ),
    (
        "01.1130",
        "viDiliN",
        Pada::Atmanepada,
        [
            "laBeta",
            "laBeyAtAm",
            "laBeran",
            "laBeTAH",
            "laBeyATAm",
            "laBeDvam",
            "laBeya",
            "laBevahi",
            "laBemahi",
        ],
    ),
    (
        "01.0574",
        "viDiliN",
        Pada::Atmanepada,
        [
            "seveta",
            "seveyAtAm",
            "severan",
            "seveTAH",
            "seveyATAm",
            "seveDvam",
            "seveya",
            "sevevahi",
            "sevemahi",
        ],
    ),
    (
        "01.0862",
        "viDiliN",
        Pada::Atmanepada,
        [
            "varteta",
            "varteyAtAm",
            "varteran",
            "varteTAH",
            "varteyATAm",
            "varteDvam",
            "varteya",
            "vartevahi",
            "vartemahi",
        ],
    ),
    (
        "01.0696",
        "viDiliN",
        Pada::Atmanepada,
        [
            "BAzeta",
            "BAzeyAtAm",
            "BAzeran",
            "BAzeTAH",
            "BAzeyATAm",
            "BAzeDvam",
            "BAzeya",
            "BAzevahi",
            "BAzemahi",
        ],
    ),
    (
        "01.0694",
        "viDiliN",
        Pada::Atmanepada,
        [
            "Ikzeta",
            "IkzeyAtAm",
            "Ikzeran",
            "IkzeTAH",
            "IkzeyATAm",
            "IkzeDvam",
            "Ikzeya",
            "Ikzevahi",
            "Ikzemahi",
        ],
    ),
    (
        "04.0001",
        "laT",
        Pada::Parasmaipada,
        [
            "dIvyati", "dIvyataH", "dIvyanti", "dIvyasi", "dIvyaTaH", "dIvyaTa", "dIvyAmi",
            "dIvyAvaH", "dIvyAmaH",
        ],
    ),
    (
        "04.0091",
        "laT",
        Pada::Parasmaipada,
        [
            "naSyati", "naSyataH", "naSyanti", "naSyasi", "naSyaTaH", "naSyaTa", "naSyAmi",
            "naSyAvaH", "naSyAmaH",
        ],
    ),
    (
        "04.0146",
        "laT",
        Pada::Parasmaipada,
        [
            "kupyati", "kupyataH", "kupyanti", "kupyasi", "kupyaTaH", "kupyaTa", "kupyAmi",
            "kupyAvaH", "kupyAmaH",
        ],
    ),
    (
        "04.0073",
        "laT",
        Pada::Atmanepada,
        [
            "manyate",
            "manyete",
            "manyante",
            "manyase",
            "manyeTe",
            "manyaDve",
            "manye",
            "manyAvahe",
            "manyAmahe",
        ],
    ),
    (
        "04.0069",
        "laT",
        Pada::Atmanepada,
        [
            "yuDyate",
            "yuDyete",
            "yuDyante",
            "yuDyase",
            "yuDyeTe",
            "yuDyaDve",
            "yuDye",
            "yuDyAvahe",
            "yuDyAmahe",
        ],
    ),
    (
        "04.0067",
        "laT",
        Pada::Atmanepada,
        [
            "vidyate",
            "vidyete",
            "vidyante",
            "vidyase",
            "vidyeTe",
            "vidyaDve",
            "vidye",
            "vidyAvahe",
            "vidyAmahe",
        ],
    ),
    (
        "06.0001",
        "laT",
        Pada::Parasmaipada,
        [
            "tudati", "tudataH", "tudanti", "tudasi", "tudaTaH", "tudaTa", "tudAmi", "tudAvaH",
            "tudAmaH",
        ],
    ),
    (
        "06.0001",
        "laT",
        Pada::Atmanepada,
        [
            "tudate", "tudete", "tudante", "tudase", "tudeTe", "tudaDve", "tude", "tudAvahe",
            "tudAmahe",
        ],
    ),
    (
        "06.0092",
        "laT",
        Pada::Parasmaipada,
        [
            "liKati", "liKataH", "liKanti", "liKasi", "liKaTaH", "liKaTa", "liKAmi", "liKAvaH",
            "liKAmaH",
        ],
    ),
    (
        "06.0160",
        "laT",
        Pada::Parasmaipada,
        [
            "viSati", "viSataH", "viSanti", "viSasi", "viSaTaH", "viSaTa", "viSAmi", "viSAvaH",
            "viSAmaH",
        ],
    ),
    (
        "06.0008",
        "laT",
        Pada::Atmanepada,
        [
            "juzate", "juzete", "juzante", "juzase", "juzeTe", "juzaDve", "juze", "juzAvahe",
            "juzAmahe",
        ],
    ),
    (
        "06.0009",
        "laT",
        Pada::Atmanepada,
        [
            "vijate", "vijete", "vijante", "vijase", "vijeTe", "vijaDve", "vije", "vijAvahe",
            "vijAmahe",
        ],
    ),
    (
        "06.0131",
        "laT",
        Pada::Atmanepada,
        [
            "gurate", "gurete", "gurante", "gurase", "gureTe", "guraDve", "gure", "gurAvahe",
            "gurAmahe",
        ],
    ),
    (
        "04.0001",
        "laN",
        Pada::Parasmaipada,
        [
            "adIvyad",
            "adIvyatAm",
            "adIvyan",
            "adIvyaH",
            "adIvyatam",
            "adIvyata",
            "adIvyam",
            "adIvyAva",
            "adIvyAma",
        ],
    ),
    (
        "04.0091",
        "laN",
        Pada::Parasmaipada,
        [
            "anaSyad",
            "anaSyatAm",
            "anaSyan",
            "anaSyaH",
            "anaSyatam",
            "anaSyata",
            "anaSyam",
            "anaSyAva",
            "anaSyAma",
        ],
    ),
    (
        "04.0146",
        "laN",
        Pada::Parasmaipada,
        [
            "akupyad",
            "akupyatAm",
            "akupyan",
            "akupyaH",
            "akupyatam",
            "akupyata",
            "akupyam",
            "akupyAva",
            "akupyAma",
        ],
    ),
    (
        "04.0073",
        "laN",
        Pada::Atmanepada,
        [
            "amanyata",
            "amanyetAm",
            "amanyanta",
            "amanyaTAH",
            "amanyeTAm",
            "amanyaDvam",
            "amanye",
            "amanyAvahi",
            "amanyAmahi",
        ],
    ),
    (
        "04.0069",
        "laN",
        Pada::Atmanepada,
        [
            "ayuDyata",
            "ayuDyetAm",
            "ayuDyanta",
            "ayuDyaTAH",
            "ayuDyeTAm",
            "ayuDyaDvam",
            "ayuDye",
            "ayuDyAvahi",
            "ayuDyAmahi",
        ],
    ),
    (
        "04.0067",
        "laN",
        Pada::Atmanepada,
        [
            "avidyata",
            "avidyetAm",
            "avidyanta",
            "avidyaTAH",
            "avidyeTAm",
            "avidyaDvam",
            "avidye",
            "avidyAvahi",
            "avidyAmahi",
        ],
    ),
    (
        "06.0001",
        "laN",
        Pada::Parasmaipada,
        [
            "atudad", "atudatAm", "atudan", "atudaH", "atudatam", "atudata", "atudam", "atudAva",
            "atudAma",
        ],
    ),
    (
        "06.0001",
        "laN",
        Pada::Atmanepada,
        [
            "atudata",
            "atudetAm",
            "atudanta",
            "atudaTAH",
            "atudeTAm",
            "atudaDvam",
            "atude",
            "atudAvahi",
            "atudAmahi",
        ],
    ),
    (
        "06.0092",
        "laN",
        Pada::Parasmaipada,
        [
            "aliKad", "aliKatAm", "aliKan", "aliKaH", "aliKatam", "aliKata", "aliKam", "aliKAva",
            "aliKAma",
        ],
    ),
    (
        "06.0160",
        "laN",
        Pada::Parasmaipada,
        [
            "aviSad", "aviSatAm", "aviSan", "aviSaH", "aviSatam", "aviSata", "aviSam", "aviSAva",
            "aviSAma",
        ],
    ),
    (
        "06.0008",
        "laN",
        Pada::Atmanepada,
        [
            "ajuzata",
            "ajuzetAm",
            "ajuzanta",
            "ajuzaTAH",
            "ajuzeTAm",
            "ajuzaDvam",
            "ajuze",
            "ajuzAvahi",
            "ajuzAmahi",
        ],
    ),
    (
        "06.0009",
        "laN",
        Pada::Atmanepada,
        [
            "avijata",
            "avijetAm",
            "avijanta",
            "avijaTAH",
            "avijeTAm",
            "avijaDvam",
            "avije",
            "avijAvahi",
            "avijAmahi",
        ],
    ),
    (
        "06.0131",
        "laN",
        Pada::Atmanepada,
        [
            "agurata",
            "aguretAm",
            "aguranta",
            "aguraTAH",
            "agureTAm",
            "aguraDvam",
            "agure",
            "agurAvahi",
            "agurAmahi",
        ],
    ),
    (
        "04.0001",
        "loT",
        Pada::Parasmaipada,
        [
            "dIvyatu", "dIvyatAm", "dIvyantu", "dIvya", "dIvyatam", "dIvyata", "dIvyAni",
            "dIvyAva", "dIvyAma",
        ],
    ),
    (
        "04.0091",
        "loT",
        Pada::Parasmaipada,
        [
            "naSyatu", "naSyatAm", "naSyantu", "naSya", "naSyatam", "naSyata", "naSyAni",
            "naSyAva", "naSyAma",
        ],
    ),
    (
        "04.0146",
        "loT",
        Pada::Parasmaipada,
        [
            "kupyatu", "kupyatAm", "kupyantu", "kupya", "kupyatam", "kupyata", "kupyAni",
            "kupyAva", "kupyAma",
        ],
    ),
    (
        "04.0073",
        "loT",
        Pada::Atmanepada,
        [
            "manyatAm",
            "manyetAm",
            "manyantAm",
            "manyasva",
            "manyeTAm",
            "manyaDvam",
            "manyE",
            "manyAvahE",
            "manyAmahE",
        ],
    ),
    (
        "04.0069",
        "loT",
        Pada::Atmanepada,
        [
            "yuDyatAm",
            "yuDyetAm",
            "yuDyantAm",
            "yuDyasva",
            "yuDyeTAm",
            "yuDyaDvam",
            "yuDyE",
            "yuDyAvahE",
            "yuDyAmahE",
        ],
    ),
    (
        "04.0067",
        "loT",
        Pada::Atmanepada,
        [
            "vidyatAm",
            "vidyetAm",
            "vidyantAm",
            "vidyasva",
            "vidyeTAm",
            "vidyaDvam",
            "vidyE",
            "vidyAvahE",
            "vidyAmahE",
        ],
    ),
    (
        "06.0001",
        "loT",
        Pada::Parasmaipada,
        [
            "tudatu", "tudatAm", "tudantu", "tuda", "tudatam", "tudata", "tudAni", "tudAva",
            "tudAma",
        ],
    ),
    (
        "06.0001",
        "loT",
        Pada::Atmanepada,
        [
            "tudatAm", "tudetAm", "tudantAm", "tudasva", "tudeTAm", "tudaDvam", "tudE", "tudAvahE",
            "tudAmahE",
        ],
    ),
    (
        "06.0092",
        "loT",
        Pada::Parasmaipada,
        [
            "liKatu", "liKatAm", "liKantu", "liKa", "liKatam", "liKata", "liKAni", "liKAva",
            "liKAma",
        ],
    ),
    (
        "06.0160",
        "loT",
        Pada::Parasmaipada,
        [
            "viSatu", "viSatAm", "viSantu", "viSa", "viSatam", "viSata", "viSAni", "viSAva",
            "viSAma",
        ],
    ),
    (
        "06.0008",
        "loT",
        Pada::Atmanepada,
        [
            "juzatAm", "juzetAm", "juzantAm", "juzasva", "juzeTAm", "juzaDvam", "juzE", "juzAvahE",
            "juzAmahE",
        ],
    ),
    (
        "06.0009",
        "loT",
        Pada::Atmanepada,
        [
            "vijatAm", "vijetAm", "vijantAm", "vijasva", "vijeTAm", "vijaDvam", "vijE", "vijAvahE",
            "vijAmahE",
        ],
    ),
    (
        "06.0131",
        "loT",
        Pada::Atmanepada,
        [
            "guratAm", "guretAm", "gurantAm", "gurasva", "gureTAm", "guraDvam", "gurE", "gurAvahE",
            "gurAmahE",
        ],
    ),
    (
        "04.0001",
        "viDiliN",
        Pada::Parasmaipada,
        [
            "dIvyed", "dIvyetAm", "dIvyeyuH", "dIvyeH", "dIvyetam", "dIvyeta", "dIvyeyam",
            "dIvyeva", "dIvyema",
        ],
    ),
    (
        "04.0091",
        "viDiliN",
        Pada::Parasmaipada,
        [
            "naSyed", "naSyetAm", "naSyeyuH", "naSyeH", "naSyetam", "naSyeta", "naSyeyam",
            "naSyeva", "naSyema",
        ],
    ),
    (
        "04.0146",
        "viDiliN",
        Pada::Parasmaipada,
        [
            "kupyed", "kupyetAm", "kupyeyuH", "kupyeH", "kupyetam", "kupyeta", "kupyeyam",
            "kupyeva", "kupyema",
        ],
    ),
    (
        "04.0073",
        "viDiliN",
        Pada::Atmanepada,
        [
            "manyeta",
            "manyeyAtAm",
            "manyeran",
            "manyeTAH",
            "manyeyATAm",
            "manyeDvam",
            "manyeya",
            "manyevahi",
            "manyemahi",
        ],
    ),
    (
        "04.0069",
        "viDiliN",
        Pada::Atmanepada,
        [
            "yuDyeta",
            "yuDyeyAtAm",
            "yuDyeran",
            "yuDyeTAH",
            "yuDyeyATAm",
            "yuDyeDvam",
            "yuDyeya",
            "yuDyevahi",
            "yuDyemahi",
        ],
    ),
    (
        "04.0067",
        "viDiliN",
        Pada::Atmanepada,
        [
            "vidyeta",
            "vidyeyAtAm",
            "vidyeran",
            "vidyeTAH",
            "vidyeyATAm",
            "vidyeDvam",
            "vidyeya",
            "vidyevahi",
            "vidyemahi",
        ],
    ),
    (
        "06.0001",
        "viDiliN",
        Pada::Parasmaipada,
        [
            "tuded", "tudetAm", "tudeyuH", "tudeH", "tudetam", "tudeta", "tudeyam", "tudeva",
            "tudema",
        ],
    ),
    (
        "06.0001",
        "viDiliN",
        Pada::Atmanepada,
        [
            "tudeta",
            "tudeyAtAm",
            "tuderan",
            "tudeTAH",
            "tudeyATAm",
            "tudeDvam",
            "tudeya",
            "tudevahi",
            "tudemahi",
        ],
    ),
    (
        "06.0092",
        "viDiliN",
        Pada::Parasmaipada,
        [
            "liKed", "liKetAm", "liKeyuH", "liKeH", "liKetam", "liKeta", "liKeyam", "liKeva",
            "liKema",
        ],
    ),
    (
        "06.0160",
        "viDiliN",
        Pada::Parasmaipada,
        [
            "viSed", "viSetAm", "viSeyuH", "viSeH", "viSetam", "viSeta", "viSeyam", "viSeva",
            "viSema",
        ],
    ),
    (
        "06.0008",
        "viDiliN",
        Pada::Atmanepada,
        [
            "juzeta",
            "juzeyAtAm",
            "juzeran",
            "juzeTAH",
            "juzeyATAm",
            "juzeDvam",
            "juzeya",
            "juzevahi",
            "juzemahi",
        ],
    ),
    (
        "06.0009",
        "viDiliN",
        Pada::Atmanepada,
        [
            "vijeta",
            "vijeyAtAm",
            "vijeran",
            "vijeTAH",
            "vijeyATAm",
            "vijeDvam",
            "vijeya",
            "vijevahi",
            "vijemahi",
        ],
    ),
    (
        "06.0131",
        "viDiliN",
        Pada::Atmanepada,
        [
            "gureta",
            "gureyAtAm",
            "gureran",
            "gureTAH",
            "gureyATAm",
            "gureDvam",
            "gureya",
            "gurevahi",
            "guremahi",
        ],
    ),
    (
        "02.0044",
        "laT",
        Pada::Parasmaipada,
        [
            "yAti", "yAtaH", "yAnti", "yAsi", "yATaH", "yATa", "yAmi", "yAvaH", "yAmaH",
        ],
    ),
    (
        "02.0045",
        "laT",
        Pada::Parasmaipada,
        [
            "vAti", "vAtaH", "vAnti", "vAsi", "vATaH", "vATa", "vAmi", "vAvaH", "vAmaH",
        ],
    ),
    (
        "02.0044",
        "laN",
        Pada::Parasmaipada,
        [
            "ayAd", "ayAtAm", "ayAn", "ayAH", "ayAtam", "ayAta", "ayAm", "ayAva", "ayAma",
        ],
    ),
    (
        "02.0045",
        "laN",
        Pada::Parasmaipada,
        [
            "avAd", "avAtAm", "avAn", "avAH", "avAtam", "avAta", "avAm", "avAva", "avAma",
        ],
    ),
    (
        "02.0044",
        "loT",
        Pada::Parasmaipada,
        [
            "yAtu", "yAtAm", "yAntu", "yAhi", "yAtam", "yAta", "yAni", "yAva", "yAma",
        ],
    ),
    (
        "02.0045",
        "loT",
        Pada::Parasmaipada,
        [
            "vAtu", "vAtAm", "vAntu", "vAhi", "vAtam", "vAta", "vAni", "vAva", "vAma",
        ],
    ),
    (
        "02.0044",
        "viDiliN",
        Pada::Parasmaipada,
        [
            "yAyAd", "yAyAtAm", "yAyuH", "yAyAH", "yAyAtam", "yAyAta", "yAyAm", "yAyAva", "yAyAma",
        ],
    ),
    (
        "02.0045",
        "viDiliN",
        Pada::Parasmaipada,
        [
            "vAyAd", "vAyAtAm", "vAyuH", "vAyAH", "vAyAtam", "vAyAta", "vAyAm", "vAyAva", "vAyAma",
        ],
    ),
    (
        "02.0001",
        "laT",
        Pada::Parasmaipada,
        [
            "atti", "attaH", "adanti", "atsi", "atTaH", "atTa", "admi", "advaH", "admaH",
        ],
    ),
    (
        "02.0001",
        "laN",
        Pada::Parasmaipada,
        [
            "Adad", "AttAm", "Adan", "AdaH", "Attam", "Atta", "Adam", "Adva", "Adma",
        ],
    ),
    (
        "02.0001",
        "loT",
        Pada::Parasmaipada,
        [
            "attu", "attAm", "adantu", "adDi", "attam", "atta", "adAni", "adAva", "adAma",
        ],
    ),
    (
        "02.0001",
        "viDiliN",
        Pada::Parasmaipada,
        [
            "adyAd", "adyAtAm", "adyuH", "adyAH", "adyAtam", "adyAta", "adyAm", "adyAva", "adyAma",
        ],
    ),
    (
        "02.0011",
        "laT",
        Pada::Atmanepada,
        [
            "Aste", "AsAte", "Asate", "Asse", "AsATe", "ADve", "Ase", "Asvahe", "Asmahe",
        ],
    ),
    (
        "02.0011",
        "laN",
        Pada::Atmanepada,
        [
            "Asta", "AsAtAm", "Asata", "AsTAH", "AsATAm", "ADvam", "Asi", "Asvahi", "Asmahi",
        ],
    ),
    (
        "02.0011",
        "loT",
        Pada::Atmanepada,
        [
            "AstAm", "AsAtAm", "AsatAm", "Assva", "AsATAm", "ADvam", "AsE", "AsAvahE", "AsAmahE",
        ],
    ),
    (
        "02.0011",
        "viDiliN",
        Pada::Atmanepada,
        [
            "AsIta", "AsIyAtAm", "AsIran", "AsITAH", "AsIyATAm", "AsIDvam", "AsIya", "AsIvahi",
            "AsImahi",
        ],
    ),
    (
        "02.0013",
        "laT",
        Pada::Atmanepada,
        [
            "vaste", "vasAte", "vasate", "vasse", "vasATe", "vaDve", "vase", "vasvahe", "vasmahe",
        ],
    ),
    (
        "02.0013",
        "laN",
        Pada::Atmanepada,
        [
            "avasta", "avasAtAm", "avasata", "avasTAH", "avasATAm", "avaDvam", "avasi", "avasvahi",
            "avasmahi",
        ],
    ),
    (
        "02.0013",
        "loT",
        Pada::Atmanepada,
        [
            "vastAm", "vasAtAm", "vasatAm", "vassva", "vasATAm", "vaDvam", "vasE", "vasAvahE",
            "vasAmahE",
        ],
    ),
    (
        "02.0013",
        "viDiliN",
        Pada::Atmanepada,
        [
            "vasIta",
            "vasIyAtAm",
            "vasIran",
            "vasITAH",
            "vasIyATAm",
            "vasIDvam",
            "vasIya",
            "vasIvahi",
            "vasImahi",
        ],
    ),
    (
        "02.0026",
        "laT",
        Pada::Atmanepada,
        [
            "Sete", "SayAte", "Serate", "Seze", "SayATe", "SeDve", "Saye", "Sevahe", "Semahe",
        ],
    ),
    (
        "02.0026",
        "laN",
        Pada::Atmanepada,
        [
            "aSeta", "aSayAtAm", "aSerata", "aSeTAH", "aSayATAm", "aSeDvam", "aSayi", "aSevahi",
            "aSemahi",
        ],
    ),
    (
        "02.0026",
        "loT",
        Pada::Atmanepada,
        [
            "SetAm", "SayAtAm", "SeratAm", "Sezva", "SayATAm", "SeDvam", "SayE", "SayAvahE",
            "SayAmahE",
        ],
    ),
    (
        "02.0026",
        "viDiliN",
        Pada::Atmanepada,
        [
            "SayIta",
            "SayIyAtAm",
            "SayIran",
            "SayITAH",
            "SayIyATAm",
            "SayIDvam",
            "SayIya",
            "SayIvahi",
            "SayImahi",
        ],
    ),
    (
        "09.0058",
        "laT",
        Pada::Parasmaipada,
        [
            "kliSnAti",
            "kliSnItaH",
            "kliSnanti",
            "kliSnAsi",
            "kliSnITaH",
            "kliSnITa",
            "kliSnAmi",
            "kliSnIvaH",
            "kliSnImaH",
        ],
    ),
    (
        "09.0058",
        "laN",
        Pada::Parasmaipada,
        [
            "akliSnAd",
            "akliSnItAm",
            "akliSnan",
            "akliSnAH",
            "akliSnItam",
            "akliSnIta",
            "akliSnAm",
            "akliSnIva",
            "akliSnIma",
        ],
    ),
    (
        "09.0058",
        "loT",
        Pada::Parasmaipada,
        [
            "kliSnAtu",
            "kliSnItAm",
            "kliSnantu",
            "kliSAna",
            "kliSnItam",
            "kliSnIta",
            "kliSnAni",
            "kliSnAva",
            "kliSnAma",
        ],
    ),
    (
        "09.0058",
        "viDiliN",
        Pada::Parasmaipada,
        [
            "kliSnIyAd",
            "kliSnIyAtAm",
            "kliSnIyuH",
            "kliSnIyAH",
            "kliSnIyAtam",
            "kliSnIyAta",
            "kliSnIyAm",
            "kliSnIyAva",
            "kliSnIyAma",
        ],
    ),
    (
        "09.0053",
        "laT",
        Pada::Parasmaipada,
        [
            "guDnAti", "guDnItaH", "guDnanti", "guDnAsi", "guDnITaH", "guDnITa", "guDnAmi",
            "guDnIvaH", "guDnImaH",
        ],
    ),
    (
        "09.0053",
        "laN",
        Pada::Parasmaipada,
        [
            "aguDnAd",
            "aguDnItAm",
            "aguDnan",
            "aguDnAH",
            "aguDnItam",
            "aguDnIta",
            "aguDnAm",
            "aguDnIva",
            "aguDnIma",
        ],
    ),
    (
        "09.0053",
        "loT",
        Pada::Parasmaipada,
        [
            "guDnAtu", "guDnItAm", "guDnantu", "guDAna", "guDnItam", "guDnIta", "guDnAni",
            "guDnAva", "guDnAma",
        ],
    ),
    (
        "09.0053",
        "viDiliN",
        Pada::Parasmaipada,
        [
            "guDnIyAd",
            "guDnIyAtAm",
            "guDnIyuH",
            "guDnIyAH",
            "guDnIyAtam",
            "guDnIyAta",
            "guDnIyAm",
            "guDnIyAva",
            "guDnIyAma",
        ],
    ),
    (
        "09.0059",
        "laT",
        Pada::Parasmaipada,
        [
            "aSnAti", "aSnItaH", "aSnanti", "aSnAsi", "aSnITaH", "aSnITa", "aSnAmi", "aSnIvaH",
            "aSnImaH",
        ],
    ),
    (
        "09.0059",
        "laN",
        Pada::Parasmaipada,
        [
            "ASnAd", "ASnItAm", "ASnan", "ASnAH", "ASnItam", "ASnIta", "ASnAm", "ASnIva", "ASnIma",
        ],
    ),
    (
        "09.0059",
        "loT",
        Pada::Parasmaipada,
        [
            "aSnAtu", "aSnItAm", "aSnantu", "aSAna", "aSnItam", "aSnIta", "aSnAni", "aSnAva",
            "aSnAma",
        ],
    ),
    (
        "09.0059",
        "viDiliN",
        Pada::Parasmaipada,
        [
            "aSnIyAd",
            "aSnIyAtAm",
            "aSnIyuH",
            "aSnIyAH",
            "aSnIyAtam",
            "aSnIyAta",
            "aSnIyAm",
            "aSnIyAva",
            "aSnIyAma",
        ],
    ),
    (
        "09.0066",
        "laT",
        Pada::Parasmaipada,
        [
            "muzRAti", "muzRItaH", "muzRanti", "muzRAsi", "muzRITaH", "muzRITa", "muzRAmi",
            "muzRIvaH", "muzRImaH",
        ],
    ),
    (
        "09.0066",
        "laN",
        Pada::Parasmaipada,
        [
            "amuzRAd",
            "amuzRItAm",
            "amuzRan",
            "amuzRAH",
            "amuzRItam",
            "amuzRIta",
            "amuzRAm",
            "amuzRIva",
            "amuzRIma",
        ],
    ),
    (
        "09.0066",
        "loT",
        Pada::Parasmaipada,
        [
            "muzRAtu", "muzRItAm", "muzRantu", "muzARa", "muzRItam", "muzRIta", "muzRAni",
            "muzRAva", "muzRAma",
        ],
    ),
    (
        "09.0066",
        "viDiliN",
        Pada::Parasmaipada,
        [
            "muzRIyAd",
            "muzRIyAtAm",
            "muzRIyuH",
            "muzRIyAH",
            "muzRIyAtam",
            "muzRIyAta",
            "muzRIyAm",
            "muzRIyAva",
            "muzRIyAma",
        ],
    ),
    (
        "09.0040",
        "laT",
        Pada::Parasmaipada,
        [
            "vrIRAti", "vrIRItaH", "vrIRanti", "vrIRAsi", "vrIRITaH", "vrIRITa", "vrIRAmi",
            "vrIRIvaH", "vrIRImaH",
        ],
    ),
    (
        "09.0040",
        "laN",
        Pada::Parasmaipada,
        [
            "avrIRAd",
            "avrIRItAm",
            "avrIRan",
            "avrIRAH",
            "avrIRItam",
            "avrIRIta",
            "avrIRAm",
            "avrIRIva",
            "avrIRIma",
        ],
    ),
    (
        "09.0040",
        "loT",
        Pada::Parasmaipada,
        [
            "vrIRAtu", "vrIRItAm", "vrIRantu", "vrIRIhi", "vrIRItam", "vrIRIta", "vrIRAni",
            "vrIRAva", "vrIRAma",
        ],
    ),
    (
        "09.0040",
        "viDiliN",
        Pada::Parasmaipada,
        [
            "vrIRIyAd",
            "vrIRIyAtAm",
            "vrIRIyuH",
            "vrIRIyAH",
            "vrIRIyAtam",
            "vrIRIyAta",
            "vrIRIyAm",
            "vrIRIyAva",
            "vrIRIyAma",
        ],
    ),
    (
        "09.0045",
        "laT",
        Pada::Atmanepada,
        [
            "vfRIte", "vfRAte", "vfRate", "vfRIze", "vfRATe", "vfRIDve", "vfRe", "vfRIvahe",
            "vfRImahe",
        ],
    ),
    (
        "09.0045",
        "laN",
        Pada::Atmanepada,
        [
            "avfRIta",
            "avfRAtAm",
            "avfRata",
            "avfRITAH",
            "avfRATAm",
            "avfRIDvam",
            "avfRi",
            "avfRIvahi",
            "avfRImahi",
        ],
    ),
    (
        "09.0045",
        "loT",
        Pada::Atmanepada,
        [
            "vfRItAm", "vfRAtAm", "vfRatAm", "vfRIzva", "vfRATAm", "vfRIDvam", "vfRE", "vfRAvahE",
            "vfRAmahE",
        ],
    ),
    (
        "09.0045",
        "viDiliN",
        Pada::Atmanepada,
        [
            "vfRIta",
            "vfRIyAtAm",
            "vfRIran",
            "vfRITAH",
            "vfRIyATAm",
            "vfRIDvam",
            "vfRIya",
            "vfRIvahi",
            "vfRImahi",
        ],
    ),
    (
        "05.0016",
        "laT",
        Pada::Parasmaipada,
        [
            "Apnoti",
            "ApnutaH",
            "Apnuvanti",
            "Apnozi",
            "ApnuTaH",
            "ApnuTa",
            "Apnomi",
            "ApnuvaH",
            "ApnumaH",
        ],
    ),
    (
        "05.0016",
        "laN",
        Pada::Parasmaipada,
        [
            "Apnod", "ApnutAm", "Apnuvan", "ApnoH", "Apnutam", "Apnuta", "Apnavam", "Apnuva",
            "Apnuma",
        ],
    ),
    (
        "05.0016",
        "loT",
        Pada::Parasmaipada,
        [
            "Apnotu",
            "ApnutAm",
            "Apnuvantu",
            "Apnuhi",
            "Apnutam",
            "Apnuta",
            "ApnavAni",
            "ApnavAva",
            "ApnavAma",
        ],
    ),
    (
        "05.0016",
        "viDiliN",
        Pada::Parasmaipada,
        [
            "ApnuyAd",
            "ApnuyAtAm",
            "ApnuyuH",
            "ApnuyAH",
            "ApnuyAtam",
            "ApnuyAta",
            "ApnuyAm",
            "ApnuyAva",
            "ApnuyAma",
        ],
    ),
    (
        "05.0017",
        "laT",
        Pada::Parasmaipada,
        [
            "Saknoti",
            "SaknutaH",
            "Saknuvanti",
            "Saknozi",
            "SaknuTaH",
            "SaknuTa",
            "Saknomi",
            "SaknuvaH",
            "SaknumaH",
        ],
    ),
    (
        "05.0017",
        "laN",
        Pada::Parasmaipada,
        [
            "aSaknod",
            "aSaknutAm",
            "aSaknuvan",
            "aSaknoH",
            "aSaknutam",
            "aSaknuta",
            "aSaknavam",
            "aSaknuva",
            "aSaknuma",
        ],
    ),
    (
        "05.0017",
        "loT",
        Pada::Parasmaipada,
        [
            "Saknotu",
            "SaknutAm",
            "Saknuvantu",
            "Saknuhi",
            "Saknutam",
            "Saknuta",
            "SaknavAni",
            "SaknavAva",
            "SaknavAma",
        ],
    ),
    (
        "05.0017",
        "viDiliN",
        Pada::Parasmaipada,
        [
            "SaknuyAd",
            "SaknuyAtAm",
            "SaknuyuH",
            "SaknuyAH",
            "SaknuyAtam",
            "SaknuyAta",
            "SaknuyAm",
            "SaknuyAva",
            "SaknuyAma",
        ],
    ),
    (
        "05.0012",
        "laT",
        Pada::Parasmaipada,
        [
            "hinoti", "hinutaH", "hinvanti", "hinozi", "hinuTaH", "hinuTa", "hinomi", "hinuvaH",
            "hinumaH",
        ],
    ),
    (
        "05.0012",
        "laN",
        Pada::Parasmaipada,
        [
            "ahinod", "ahinutAm", "ahinvan", "ahinoH", "ahinutam", "ahinuta", "ahinavam",
            "ahinuva", "ahinuma",
        ],
    ),
    (
        "05.0012",
        "loT",
        Pada::Parasmaipada,
        [
            "hinotu", "hinutAm", "hinvantu", "hinu", "hinutam", "hinuta", "hinavAni", "hinavAva",
            "hinavAma",
        ],
    ),
    (
        "05.0012",
        "viDiliN",
        Pada::Parasmaipada,
        [
            "hinuyAd",
            "hinuyAtAm",
            "hinuyuH",
            "hinuyAH",
            "hinuyAtam",
            "hinuyAta",
            "hinuyAm",
            "hinuyAva",
            "hinuyAma",
        ],
    ),
    (
        "05.0032",
        "laT",
        Pada::Parasmaipada,
        [
            "riRoti", "riRutaH", "riRvanti", "riRozi", "riRuTaH", "riRuTa", "riRomi", "riRuvaH",
            "riRumaH",
        ],
    ),
    (
        "05.0032",
        "laN",
        Pada::Parasmaipada,
        [
            "ariRod", "ariRutAm", "ariRvan", "ariRoH", "ariRutam", "ariRuta", "ariRavam",
            "ariRuva", "ariRuma",
        ],
    ),
    (
        "05.0032",
        "loT",
        Pada::Parasmaipada,
        [
            "riRotu", "riRutAm", "riRvantu", "riRu", "riRutam", "riRuta", "riRavAni", "riRavAva",
            "riRavAma",
        ],
    ),
    (
        "05.0032",
        "viDiliN",
        Pada::Parasmaipada,
        [
            "riRuyAd",
            "riRuyAtAm",
            "riRuyuH",
            "riRuyAH",
            "riRuyAtam",
            "riRuyAta",
            "riRuyAm",
            "riRuyAva",
            "riRuyAma",
        ],
    ),
    (
        "05.0020",
        "laT",
        Pada::Atmanepada,
        [
            "aSnute", "aSnuvAte", "aSnuvate", "aSnuze", "aSnuvATe", "aSnuDve", "aSnuve",
            "aSnuvahe", "aSnumahe",
        ],
    ),
    (
        "05.0020",
        "laN",
        Pada::Atmanepada,
        [
            "ASnuta",
            "ASnuvAtAm",
            "ASnuvata",
            "ASnuTAH",
            "ASnuvATAm",
            "ASnuDvam",
            "ASnuvi",
            "ASnuvahi",
            "ASnumahi",
        ],
    ),
    (
        "05.0020",
        "loT",
        Pada::Atmanepada,
        [
            "aSnutAm",
            "aSnuvAtAm",
            "aSnuvatAm",
            "aSnuzva",
            "aSnuvATAm",
            "aSnuDvam",
            "aSnavE",
            "aSnavAvahE",
            "aSnavAmahE",
        ],
    ),
    (
        "05.0020",
        "viDiliN",
        Pada::Atmanepada,
        [
            "aSnuvIta",
            "aSnuvIyAtAm",
            "aSnuvIran",
            "aSnuvITAH",
            "aSnuvIyATAm",
            "aSnuvIDvam",
            "aSnuvIya",
            "aSnuvIvahi",
            "aSnuvImahi",
        ],
    ),
    (
        "05.0021",
        "laT",
        Pada::Atmanepada,
        [
            "stiGnute",
            "stiGnuvAte",
            "stiGnuvate",
            "stiGnuze",
            "stiGnuvATe",
            "stiGnuDve",
            "stiGnuve",
            "stiGnuvahe",
            "stiGnumahe",
        ],
    ),
    (
        "05.0021",
        "laN",
        Pada::Atmanepada,
        [
            "astiGnuta",
            "astiGnuvAtAm",
            "astiGnuvata",
            "astiGnuTAH",
            "astiGnuvATAm",
            "astiGnuDvam",
            "astiGnuvi",
            "astiGnuvahi",
            "astiGnumahi",
        ],
    ),
    (
        "05.0021",
        "loT",
        Pada::Atmanepada,
        [
            "stiGnutAm",
            "stiGnuvAtAm",
            "stiGnuvatAm",
            "stiGnuzva",
            "stiGnuvATAm",
            "stiGnuDvam",
            "stiGnavE",
            "stiGnavAvahE",
            "stiGnavAmahE",
        ],
    ),
    (
        "05.0021",
        "viDiliN",
        Pada::Atmanepada,
        [
            "stiGnuvIta",
            "stiGnuvIyAtAm",
            "stiGnuvIran",
            "stiGnuvITAH",
            "stiGnuvIyATAm",
            "stiGnuvIDvam",
            "stiGnuvIya",
            "stiGnuvIvahi",
            "stiGnuvImahi",
        ],
    ),
    (
        "07.0010",
        "laT",
        Pada::Parasmaipada,
        [
            "kfRatti", "kfnttaH", "kfntanti", "kfRatsi", "kfntTaH", "kfntTa", "kfRatmi", "kfntvaH",
            "kfntmaH",
        ],
    ),
    (
        "07.0010",
        "laN",
        Pada::Parasmaipada,
        [
            "akfRad", "akfnttAm", "akfntan", "akfRad", "akfnttam", "akfntta", "akfRatam",
            "akfntva", "akfntma",
        ],
    ),
    (
        "07.0010",
        "loT",
        Pada::Parasmaipada,
        [
            "kfRattu", "kfnttAm", "kfntantu", "kfndDi", "kfnttam", "kfntta", "kfRatAni",
            "kfRatAva", "kfRatAma",
        ],
    ),
    (
        "07.0010",
        "viDiliN",
        Pada::Parasmaipada,
        [
            "kfntyAd",
            "kfntyAtAm",
            "kfntyuH",
            "kfntyAH",
            "kfntyAtam",
            "kfntyAta",
            "kfntyAm",
            "kfntyAva",
            "kfntyAma",
        ],
    ),
    (
        "07.0019",
        "laT",
        Pada::Parasmaipada,
        [
            "hinasti", "hiMstaH", "hiMsanti", "hinassi", "hiMsTaH", "hiMsTa", "hinasmi", "hiMsvaH",
            "hiMsmaH",
        ],
    ),
    (
        "07.0019",
        "laN",
        Pada::Parasmaipada,
        [
            "ahinad", "ahiMstAm", "ahiMsan", "ahinad", "ahiMstam", "ahiMsta", "ahinasam",
            "ahiMsva", "ahiMsma",
        ],
    ),
    (
        "07.0019",
        "loT",
        Pada::Parasmaipada,
        [
            "hinastu", "hiMstAm", "hiMsantu", "hinDi", "hiMstam", "hiMsta", "hinasAni", "hinasAva",
            "hinasAma",
        ],
    ),
    (
        "07.0019",
        "viDiliN",
        Pada::Parasmaipada,
        [
            "hiMsyAd",
            "hiMsyAtAm",
            "hiMsyuH",
            "hiMsyAH",
            "hiMsyAtam",
            "hiMsyAta",
            "hiMsyAm",
            "hiMsyAva",
            "hiMsyAma",
        ],
    ),
    (
        "07.0012",
        "laT",
        Pada::Atmanepada,
        [
            "Kintte", "KindAte", "Kindate", "Kintse", "KindATe", "KindDve", "Kinde", "Kindvahe",
            "Kindmahe",
        ],
    ),
    (
        "07.0012",
        "laN",
        Pada::Atmanepada,
        [
            "aKintta",
            "aKindAtAm",
            "aKindata",
            "aKintTAH",
            "aKindATAm",
            "aKindDvam",
            "aKindi",
            "aKindvahi",
            "aKindmahi",
        ],
    ),
    (
        "07.0012",
        "loT",
        Pada::Atmanepada,
        [
            "KinttAm",
            "KindAtAm",
            "KindatAm",
            "Kintsva",
            "KindATAm",
            "KindDvam",
            "KinadE",
            "KinadAvahE",
            "KinadAmahE",
        ],
    ),
    (
        "07.0012",
        "viDiliN",
        Pada::Atmanepada,
        [
            "KindIta",
            "KindIyAtAm",
            "KindIran",
            "KindITAH",
            "KindIyATAm",
            "KindIDvam",
            "KindIya",
            "KindIvahi",
            "KindImahi",
        ],
    ),
    (
        "07.0016",
        "laT",
        Pada::Parasmaipada,
        [
            "Banakti", "BaNktaH", "BaYjanti", "Banakzi", "BaNkTaH", "BaNkTa", "Banajmi", "BaYjvaH",
            "BaYjmaH",
        ],
    ),
    (
        "07.0016",
        "laN",
        Pada::Parasmaipada,
        [
            "aBanag", "aBaNktAm", "aBaYjan", "aBanag", "aBaNktam", "aBaNkta", "aBanajam",
            "aBaYjva", "aBaYjma",
        ],
    ),
    (
        "07.0016",
        "loT",
        Pada::Parasmaipada,
        [
            "Banaktu", "BaNktAm", "BaYjantu", "BaNgDi", "BaNktam", "BaNkta", "BanajAni",
            "BanajAva", "BanajAma",
        ],
    ),
    (
        "07.0016",
        "viDiliN",
        Pada::Parasmaipada,
        [
            "BaYjyAd",
            "BaYjyAtAm",
            "BaYjyuH",
            "BaYjyAH",
            "BaYjyAtam",
            "BaYjyAta",
            "BaYjyAm",
            "BaYjyAva",
            "BaYjyAma",
        ],
    ),
    (
        "07.0015",
        "laT",
        Pada::Parasmaipada,
        [
            "pinazwi", "piMzwaH", "piMzanti", "pinakzi", "piMzWaH", "piMzWa", "pinazmi", "piMzvaH",
            "piMzmaH",
        ],
    ),
    (
        "07.0015",
        "laN",
        Pada::Parasmaipada,
        [
            "apinaq", "apiMzwAm", "apiMzan", "apinaq", "apiMzwam", "apiMzwa", "apinazam",
            "apiMzva", "apiMzma",
        ],
    ),
    (
        "07.0015",
        "loT",
        Pada::Parasmaipada,
        [
            "pinazwu", "piMzwAm", "piMzantu", "piRqQi", "piMzwam", "piMzwa", "pinazARi",
            "pinazAva", "pinazAma",
        ],
    ),
    (
        "07.0015",
        "viDiliN",
        Pada::Parasmaipada,
        [
            "piMzyAd",
            "piMzyAtAm",
            "piMzyuH",
            "piMzyAH",
            "piMzyAtam",
            "piMzyAta",
            "piMzyAm",
            "piMzyAva",
            "piMzyAma",
        ],
    ),
    (
        "07.0011",
        "laT",
        Pada::Atmanepada,
        [
            "indDe", "inDAte", "inDate", "intse", "inDATe", "indDve", "inDe", "inDvahe", "inDmahe",
        ],
    ),
    (
        "07.0011",
        "laN",
        Pada::Atmanepada,
        [
            "EndDa", "EnDAtAm", "EnData", "EndDAH", "EnDATAm", "EndDvam", "EnDi", "EnDvahi",
            "EnDmahi",
        ],
    ),
    (
        "07.0011",
        "loT",
        Pada::Atmanepada,
        [
            "indDAm",
            "inDAtAm",
            "inDatAm",
            "intsva",
            "inDATAm",
            "indDvam",
            "inaDE",
            "inaDAvahE",
            "inaDAmahE",
        ],
    ),
    (
        "07.0011",
        "viDiliN",
        Pada::Atmanepada,
        [
            "inDIta",
            "inDIyAtAm",
            "inDIran",
            "inDITAH",
            "inDIyATAm",
            "inDIDvam",
            "inDIya",
            "inDIvahi",
            "inDImahi",
        ],
    ),
    (
        "07.0001",
        "laT",
        Pada::Parasmaipada,
        [
            "ruRadDi", "rundDaH", "runDanti", "ruRatsi", "rundDaH", "rundDa", "ruRaDmi", "runDvaH",
            "runDmaH",
        ],
    ),
    (
        "07.0001",
        "laN",
        Pada::Parasmaipada,
        [
            "aruRad", "arundDAm", "arunDan", "aruRad", "arundDam", "arundDa", "aruRaDam",
            "arunDva", "arunDma",
        ],
    ),
    (
        "07.0001",
        "loT",
        Pada::Parasmaipada,
        [
            "ruRadDu", "rundDAm", "runDantu", "rundDi", "rundDam", "rundDa", "ruRaDAni",
            "ruRaDAva", "ruRaDAma",
        ],
    ),
    (
        "07.0001",
        "viDiliN",
        Pada::Parasmaipada,
        [
            "runDyAd",
            "runDyAtAm",
            "runDyuH",
            "runDyAH",
            "runDyAtam",
            "runDyAta",
            "runDyAm",
            "runDyAva",
            "runDyAma",
        ],
    ),
    (
        "07.0001",
        "laT",
        Pada::Atmanepada,
        [
            "rundDe", "runDAte", "runDate", "runtse", "runDATe", "rundDve", "runDe", "runDvahe",
            "runDmahe",
        ],
    ),
    (
        "07.0001",
        "laN",
        Pada::Atmanepada,
        [
            "arundDa",
            "arunDAtAm",
            "arunData",
            "arundDAH",
            "arunDATAm",
            "arundDvam",
            "arunDi",
            "arunDvahi",
            "arunDmahi",
        ],
    ),
    (
        "07.0001",
        "loT",
        Pada::Atmanepada,
        [
            "rundDAm",
            "runDAtAm",
            "runDatAm",
            "runtsva",
            "runDATAm",
            "rundDvam",
            "ruRaDE",
            "ruRaDAvahE",
            "ruRaDAmahE",
        ],
    ),
    (
        "07.0001",
        "viDiliN",
        Pada::Atmanepada,
        [
            "runDIta",
            "runDIyAtAm",
            "runDIran",
            "runDITAH",
            "runDIyATAm",
            "runDIDvam",
            "runDIya",
            "runDIvahi",
            "runDImahi",
        ],
    ),
    (
        "07.0002",
        "laT",
        Pada::Parasmaipada,
        [
            "Binatti", "BinttaH", "Bindanti", "Binatsi", "BintTaH", "BintTa", "Binadmi", "BindvaH",
            "BindmaH",
        ],
    ),
    (
        "07.0002",
        "laN",
        Pada::Parasmaipada,
        [
            "aBinad", "aBinttAm", "aBindan", "aBinad", "aBinttam", "aBintta", "aBinadam",
            "aBindva", "aBindma",
        ],
    ),
    (
        "07.0002",
        "loT",
        Pada::Parasmaipada,
        [
            "Binattu", "BinttAm", "Bindantu", "BindDi", "Binttam", "Bintta", "BinadAni",
            "BinadAva", "BinadAma",
        ],
    ),
    (
        "07.0002",
        "viDiliN",
        Pada::Parasmaipada,
        [
            "BindyAd",
            "BindyAtAm",
            "BindyuH",
            "BindyAH",
            "BindyAtam",
            "BindyAta",
            "BindyAm",
            "BindyAva",
            "BindyAma",
        ],
    ),
    (
        "07.0002",
        "laT",
        Pada::Atmanepada,
        [
            "Bintte", "BindAte", "Bindate", "Bintse", "BindATe", "BindDve", "Binde", "Bindvahe",
            "Bindmahe",
        ],
    ),
    (
        "07.0002",
        "laN",
        Pada::Atmanepada,
        [
            "aBintta",
            "aBindAtAm",
            "aBindata",
            "aBintTAH",
            "aBindATAm",
            "aBindDvam",
            "aBindi",
            "aBindvahi",
            "aBindmahi",
        ],
    ),
    (
        "07.0002",
        "loT",
        Pada::Atmanepada,
        [
            "BinttAm",
            "BindAtAm",
            "BindatAm",
            "Bintsva",
            "BindATAm",
            "BindDvam",
            "BinadE",
            "BinadAvahE",
            "BinadAmahE",
        ],
    ),
    (
        "07.0002",
        "viDiliN",
        Pada::Atmanepada,
        [
            "BindIta",
            "BindIyAtAm",
            "BindIran",
            "BindITAH",
            "BindIyATAm",
            "BindIDvam",
            "BindIya",
            "BindIvahi",
            "BindImahi",
        ],
    ),
    (
        "07.0006",
        "laT",
        Pada::Parasmaipada,
        [
            "kzuRatti",
            "kzunttaH",
            "kzundanti",
            "kzuRatsi",
            "kzuntTaH",
            "kzuntTa",
            "kzuRadmi",
            "kzundvaH",
            "kzundmaH",
        ],
    ),
    (
        "07.0006",
        "laN",
        Pada::Parasmaipada,
        [
            "akzuRad",
            "akzunttAm",
            "akzundan",
            "akzuRad",
            "akzunttam",
            "akzuntta",
            "akzuRadam",
            "akzundva",
            "akzundma",
        ],
    ),
    (
        "07.0006",
        "loT",
        Pada::Parasmaipada,
        [
            "kzuRattu",
            "kzunttAm",
            "kzundantu",
            "kzundDi",
            "kzunttam",
            "kzuntta",
            "kzuRadAni",
            "kzuRadAva",
            "kzuRadAma",
        ],
    ),
    (
        "07.0006",
        "viDiliN",
        Pada::Parasmaipada,
        [
            "kzundyAd",
            "kzundyAtAm",
            "kzundyuH",
            "kzundyAH",
            "kzundyAtam",
            "kzundyAta",
            "kzundyAm",
            "kzundyAva",
            "kzundyAma",
        ],
    ),
    (
        "07.0006",
        "laT",
        Pada::Atmanepada,
        [
            "kzuntte",
            "kzundAte",
            "kzundate",
            "kzuntse",
            "kzundATe",
            "kzundDve",
            "kzunde",
            "kzundvahe",
            "kzundmahe",
        ],
    ),
    (
        "07.0006",
        "laN",
        Pada::Atmanepada,
        [
            "akzuntta",
            "akzundAtAm",
            "akzundata",
            "akzuntTAH",
            "akzundATAm",
            "akzundDvam",
            "akzundi",
            "akzundvahi",
            "akzundmahi",
        ],
    ),
    (
        "07.0006",
        "loT",
        Pada::Atmanepada,
        [
            "kzunttAm",
            "kzundAtAm",
            "kzundatAm",
            "kzuntsva",
            "kzundATAm",
            "kzundDvam",
            "kzuRadE",
            "kzuRadAvahE",
            "kzuRadAmahE",
        ],
    ),
    (
        "07.0006",
        "viDiliN",
        Pada::Atmanepada,
        [
            "kzundIta",
            "kzundIyAtAm",
            "kzundIran",
            "kzundITAH",
            "kzundIyATAm",
            "kzundIDvam",
            "kzundIya",
            "kzundIvahi",
            "kzundImahi",
        ],
    ),
    (
        "07.0007",
        "laT",
        Pada::Parasmaipada,
        [
            "yunakti", "yuNktaH", "yuYjanti", "yunakzi", "yuNkTaH", "yuNkTa", "yunajmi", "yuYjvaH",
            "yuYjmaH",
        ],
    ),
    (
        "07.0007",
        "laN",
        Pada::Parasmaipada,
        [
            "ayunag", "ayuNktAm", "ayuYjan", "ayunag", "ayuNktam", "ayuNkta", "ayunajam",
            "ayuYjva", "ayuYjma",
        ],
    ),
    (
        "07.0007",
        "loT",
        Pada::Parasmaipada,
        [
            "yunaktu", "yuNktAm", "yuYjantu", "yuNgDi", "yuNktam", "yuNkta", "yunajAni",
            "yunajAva", "yunajAma",
        ],
    ),
    (
        "07.0007",
        "viDiliN",
        Pada::Parasmaipada,
        [
            "yuYjyAd",
            "yuYjyAtAm",
            "yuYjyuH",
            "yuYjyAH",
            "yuYjyAtam",
            "yuYjyAta",
            "yuYjyAm",
            "yuYjyAva",
            "yuYjyAma",
        ],
    ),
    (
        "07.0007",
        "laT",
        Pada::Atmanepada,
        [
            "yuNkte", "yuYjAte", "yuYjate", "yuNkze", "yuYjATe", "yuNgDve", "yuYje", "yuYjvahe",
            "yuYjmahe",
        ],
    ),
    (
        "07.0007",
        "laN",
        Pada::Atmanepada,
        [
            "ayuNkta",
            "ayuYjAtAm",
            "ayuYjata",
            "ayuNkTAH",
            "ayuYjATAm",
            "ayuNgDvam",
            "ayuYji",
            "ayuYjvahi",
            "ayuYjmahi",
        ],
    ),
    (
        "07.0007",
        "loT",
        Pada::Atmanepada,
        [
            "yuNktAm",
            "yuYjAtAm",
            "yuYjatAm",
            "yuNkzva",
            "yuYjATAm",
            "yuNgDvam",
            "yunajE",
            "yunajAvahE",
            "yunajAmahE",
        ],
    ),
    (
        "07.0007",
        "viDiliN",
        Pada::Atmanepada,
        [
            "yuYjIta",
            "yuYjIyAtAm",
            "yuYjIran",
            "yuYjITAH",
            "yuYjIyATAm",
            "yuYjIDvam",
            "yuYjIya",
            "yuYjIvahi",
            "yuYjImahi",
        ],
    ),
    (
        "07.0009",
        "laT",
        Pada::Parasmaipada,
        [
            "tfRatti", "tfnttaH", "tfndanti", "tfRatsi", "tfntTaH", "tfntTa", "tfRadmi", "tfndvaH",
            "tfndmaH",
        ],
    ),
    (
        "07.0009",
        "laN",
        Pada::Parasmaipada,
        [
            "atfRad", "atfnttAm", "atfndan", "atfRad", "atfnttam", "atfntta", "atfRadam",
            "atfndva", "atfndma",
        ],
    ),
    (
        "07.0009",
        "loT",
        Pada::Parasmaipada,
        [
            "tfRattu", "tfnttAm", "tfndantu", "tfndDi", "tfnttam", "tfntta", "tfRadAni",
            "tfRadAva", "tfRadAma",
        ],
    ),
    (
        "07.0009",
        "viDiliN",
        Pada::Parasmaipada,
        [
            "tfndyAd",
            "tfndyAtAm",
            "tfndyuH",
            "tfndyAH",
            "tfndyAtam",
            "tfndyAta",
            "tfndyAm",
            "tfndyAva",
            "tfndyAma",
        ],
    ),
    (
        "07.0009",
        "laT",
        Pada::Atmanepada,
        [
            "tfntte", "tfndAte", "tfndate", "tfntse", "tfndATe", "tfndDve", "tfnde", "tfndvahe",
            "tfndmahe",
        ],
    ),
    (
        "07.0009",
        "laN",
        Pada::Atmanepada,
        [
            "atfntta",
            "atfndAtAm",
            "atfndata",
            "atfntTAH",
            "atfndATAm",
            "atfndDvam",
            "atfndi",
            "atfndvahi",
            "atfndmahi",
        ],
    ),
    (
        "07.0009",
        "loT",
        Pada::Atmanepada,
        [
            "tfnttAm",
            "tfndAtAm",
            "tfndatAm",
            "tfntsva",
            "tfndATAm",
            "tfndDvam",
            "tfRadE",
            "tfRadAvahE",
            "tfRadAmahE",
        ],
    ),
    (
        "07.0009",
        "viDiliN",
        Pada::Atmanepada,
        [
            "tfndIta",
            "tfndIyAtAm",
            "tfndIran",
            "tfndITAH",
            "tfndIyATAm",
            "tfndIDvam",
            "tfndIya",
            "tfndIvahi",
            "tfndImahi",
        ],
    ),
    (
        "07.0004",
        "laT",
        Pada::Parasmaipada,
        [
            "riRakti", "riNktaH", "riYcanti", "riRakzi", "riNkTaH", "riNkTa", "riRacmi", "riYcvaH",
            "riYcmaH",
        ],
    ),
    (
        "07.0004",
        "laN",
        Pada::Parasmaipada,
        [
            "ariRag", "ariNktAm", "ariYcan", "ariRag", "ariNktam", "ariNkta", "ariRacam",
            "ariYcva", "ariYcma",
        ],
    ),
    (
        "07.0004",
        "loT",
        Pada::Parasmaipada,
        [
            "riRaktu", "riNktAm", "riYcantu", "riNgDi", "riNktam", "riNkta", "riRacAni",
            "riRacAva", "riRacAma",
        ],
    ),
    (
        "07.0004",
        "viDiliN",
        Pada::Parasmaipada,
        [
            "riYcyAd",
            "riYcyAtAm",
            "riYcyuH",
            "riYcyAH",
            "riYcyAtam",
            "riYcyAta",
            "riYcyAm",
            "riYcyAva",
            "riYcyAma",
        ],
    ),
    (
        "07.0004",
        "laT",
        Pada::Atmanepada,
        [
            "riNkte", "riYcAte", "riYcate", "riNkze", "riYcATe", "riNgDve", "riYce", "riYcvahe",
            "riYcmahe",
        ],
    ),
    (
        "07.0004",
        "laN",
        Pada::Atmanepada,
        [
            "ariNkta",
            "ariYcAtAm",
            "ariYcata",
            "ariNkTAH",
            "ariYcATAm",
            "ariNgDvam",
            "ariYci",
            "ariYcvahi",
            "ariYcmahi",
        ],
    ),
    (
        "07.0004",
        "loT",
        Pada::Atmanepada,
        [
            "riNktAm",
            "riYcAtAm",
            "riYcatAm",
            "riNkzva",
            "riYcATAm",
            "riNgDvam",
            "riRacE",
            "riRacAvahE",
            "riRacAmahE",
        ],
    ),
    (
        "07.0004",
        "viDiliN",
        Pada::Atmanepada,
        [
            "riYcIta",
            "riYcIyAtAm",
            "riYcIran",
            "riYcITAH",
            "riYcIyATAm",
            "riYcIDvam",
            "riYcIya",
            "riYcIvahi",
            "riYcImahi",
        ],
    ),
    (
        "07.0005",
        "laT",
        Pada::Parasmaipada,
        [
            "vinakti", "viNktaH", "viYcanti", "vinakzi", "viNkTaH", "viNkTa", "vinacmi", "viYcvaH",
            "viYcmaH",
        ],
    ),
    (
        "07.0005",
        "laN",
        Pada::Parasmaipada,
        [
            "avinag", "aviNktAm", "aviYcan", "avinag", "aviNktam", "aviNkta", "avinacam",
            "aviYcva", "aviYcma",
        ],
    ),
    (
        "07.0005",
        "loT",
        Pada::Parasmaipada,
        [
            "vinaktu", "viNktAm", "viYcantu", "viNgDi", "viNktam", "viNkta", "vinacAni",
            "vinacAva", "vinacAma",
        ],
    ),
    (
        "07.0005",
        "viDiliN",
        Pada::Parasmaipada,
        [
            "viYcyAd",
            "viYcyAtAm",
            "viYcyuH",
            "viYcyAH",
            "viYcyAtam",
            "viYcyAta",
            "viYcyAm",
            "viYcyAva",
            "viYcyAma",
        ],
    ),
    (
        "07.0005",
        "laT",
        Pada::Atmanepada,
        [
            "viNkte", "viYcAte", "viYcate", "viNkze", "viYcATe", "viNgDve", "viYce", "viYcvahe",
            "viYcmahe",
        ],
    ),
    (
        "07.0005",
        "laN",
        Pada::Atmanepada,
        [
            "aviNkta",
            "aviYcAtAm",
            "aviYcata",
            "aviNkTAH",
            "aviYcATAm",
            "aviNgDvam",
            "aviYci",
            "aviYcvahi",
            "aviYcmahi",
        ],
    ),
    (
        "07.0005",
        "loT",
        Pada::Atmanepada,
        [
            "viNktAm",
            "viYcAtAm",
            "viYcatAm",
            "viNkzva",
            "viYcATAm",
            "viNgDvam",
            "vinacE",
            "vinacAvahE",
            "vinacAmahE",
        ],
    ),
    (
        "07.0005",
        "viDiliN",
        Pada::Atmanepada,
        [
            "viYcIta",
            "viYcIyAtAm",
            "viYcIran",
            "viYcITAH",
            "viYcIyATAm",
            "viYcIDvam",
            "viYcIya",
            "viYcIvahi",
            "viYcImahi",
        ],
    ),
];

/// Second and third valid forms, for cells where an optional (vikalpa) rule
/// forks the derivation. `(root_number, lakara_label, pada, cell index into the
/// [&str; 9], alternate form, vikalpa key)`.
///
/// The vikalpa key names the optional rules applied on the branch that
/// derives this form, `+`-joined in pipeline order. It is not decoration:
/// `every_alternate_names_the_vikalpa_rules_that_produced_it` checks it
/// against the branch's own log, so a right form reached by the wrong rule
/// fails here.
///
/// `PARADIGM` holds index 0 — the derivation with no optional rule applied —
/// so an alternate is by construction never `PARADIGM`'s own string.
/// Cell order is [P.E, P.D, P.B, M.E, M.D, M.B, U.E, U.D, U.B], so 7 and 8
/// are uttama dvi and uttama bahu. `pada` names the block the row belongs
/// to, same as `PARADIGM`'s column.
const ALTERNATES: &[(&str, &str, Pada, usize, &str, &str)] = &[
    ("05.0012", "laT", Pada::Parasmaipada, 7, "hinvaH", "6.4.107"),
    ("05.0012", "laT", Pada::Parasmaipada, 8, "hinmaH", "6.4.107"),
    ("05.0012", "laN", Pada::Parasmaipada, 7, "ahinva", "6.4.107"),
    ("05.0012", "laN", Pada::Parasmaipada, 8, "ahinma", "6.4.107"),
    ("05.0032", "laT", Pada::Parasmaipada, 7, "riRvaH", "6.4.107"),
    ("05.0032", "laT", Pada::Parasmaipada, 8, "riRmaH", "6.4.107"),
    ("05.0032", "laN", Pada::Parasmaipada, 7, "ariRva", "6.4.107"),
    ("05.0032", "laN", Pada::Parasmaipada, 8, "ariRma", "6.4.107"),
    ("01.0001", "laN", Pada::Parasmaipada, 0, "aBavat", "8.4.56"),
    ("01.1049", "laN", Pada::Parasmaipada, 0, "anayat", "8.4.56"),
    ("01.0642", "laN", Pada::Parasmaipada, 0, "ajayat", "8.4.56"),
    ("01.1082", "laN", Pada::Parasmaipada, 0, "asmarat", "8.4.56"),
    ("01.0381", "laN", Pada::Parasmaipada, 0, "apaWat", "8.4.56"),
    ("01.1164", "laN", Pada::Parasmaipada, 0, "avadat", "8.4.56"),
    (
        "01.0001",
        "viDiliN",
        Pada::Parasmaipada,
        0,
        "Bavet",
        "8.4.56",
    ),
    (
        "01.1049",
        "viDiliN",
        Pada::Parasmaipada,
        0,
        "nayet",
        "8.4.56",
    ),
    (
        "01.0642",
        "viDiliN",
        Pada::Parasmaipada,
        0,
        "jayet",
        "8.4.56",
    ),
    (
        "01.1082",
        "viDiliN",
        Pada::Parasmaipada,
        0,
        "smaret",
        "8.4.56",
    ),
    (
        "01.0381",
        "viDiliN",
        Pada::Parasmaipada,
        0,
        "paWet",
        "8.4.56",
    ),
    (
        "01.1164",
        "viDiliN",
        Pada::Parasmaipada,
        0,
        "vadet",
        "8.4.56",
    ),
    ("04.0001", "laN", Pada::Parasmaipada, 0, "adIvyat", "8.4.56"),
    ("04.0091", "laN", Pada::Parasmaipada, 0, "anaSyat", "8.4.56"),
    ("04.0146", "laN", Pada::Parasmaipada, 0, "akupyat", "8.4.56"),
    ("06.0001", "laN", Pada::Parasmaipada, 0, "atudat", "8.4.56"),
    ("06.0092", "laN", Pada::Parasmaipada, 0, "aliKat", "8.4.56"),
    ("06.0160", "laN", Pada::Parasmaipada, 0, "aviSat", "8.4.56"),
    (
        "04.0001",
        "viDiliN",
        Pada::Parasmaipada,
        0,
        "dIvyet",
        "8.4.56",
    ),
    (
        "04.0091",
        "viDiliN",
        Pada::Parasmaipada,
        0,
        "naSyet",
        "8.4.56",
    ),
    (
        "04.0146",
        "viDiliN",
        Pada::Parasmaipada,
        0,
        "kupyet",
        "8.4.56",
    ),
    (
        "06.0001",
        "viDiliN",
        Pada::Parasmaipada,
        0,
        "tudet",
        "8.4.56",
    ),
    (
        "06.0092",
        "viDiliN",
        Pada::Parasmaipada,
        0,
        "liKet",
        "8.4.56",
    ),
    (
        "06.0160",
        "viDiliN",
        Pada::Parasmaipada,
        0,
        "viSet",
        "8.4.56",
    ),
    ("02.0044", "laN", Pada::Parasmaipada, 0, "ayAt", "8.4.56"),
    ("02.0045", "laN", Pada::Parasmaipada, 0, "avAt", "8.4.56"),
    (
        "02.0044",
        "viDiliN",
        Pada::Parasmaipada,
        0,
        "yAyAt",
        "8.4.56",
    ),
    (
        "02.0045",
        "viDiliN",
        Pada::Parasmaipada,
        0,
        "vAyAt",
        "8.4.56",
    ),
    ("02.0001", "laN", Pada::Parasmaipada, 0, "Adat", "8.4.56"),
    (
        "02.0001",
        "viDiliN",
        Pada::Parasmaipada,
        0,
        "adyAt",
        "8.4.56",
    ),
    (
        "09.0058",
        "laN",
        Pada::Parasmaipada,
        0,
        "akliSnAt",
        "8.4.56",
    ),
    (
        "09.0058",
        "viDiliN",
        Pada::Parasmaipada,
        0,
        "kliSnIyAt",
        "8.4.56",
    ),
    ("09.0053", "laN", Pada::Parasmaipada, 0, "aguDnAt", "8.4.56"),
    (
        "09.0053",
        "viDiliN",
        Pada::Parasmaipada,
        0,
        "guDnIyAt",
        "8.4.56",
    ),
    ("09.0059", "laN", Pada::Parasmaipada, 0, "ASnAt", "8.4.56"),
    (
        "09.0059",
        "viDiliN",
        Pada::Parasmaipada,
        0,
        "aSnIyAt",
        "8.4.56",
    ),
    ("09.0066", "laN", Pada::Parasmaipada, 0, "amuzRAt", "8.4.56"),
    (
        "09.0066",
        "viDiliN",
        Pada::Parasmaipada,
        0,
        "muzRIyAt",
        "8.4.56",
    ),
    ("09.0040", "laN", Pada::Parasmaipada, 0, "avrIRAt", "8.4.56"),
    (
        "09.0040",
        "viDiliN",
        Pada::Parasmaipada,
        0,
        "vrIRIyAt",
        "8.4.56",
    ),
    ("05.0016", "laN", Pada::Parasmaipada, 0, "Apnot", "8.4.56"),
    (
        "05.0016",
        "viDiliN",
        Pada::Parasmaipada,
        0,
        "ApnuyAt",
        "8.4.56",
    ),
    ("05.0017", "laN", Pada::Parasmaipada, 0, "aSaknot", "8.4.56"),
    (
        "05.0017",
        "viDiliN",
        Pada::Parasmaipada,
        0,
        "SaknuyAt",
        "8.4.56",
    ),
    ("05.0012", "laN", Pada::Parasmaipada, 0, "ahinot", "8.4.56"),
    (
        "05.0012",
        "viDiliN",
        Pada::Parasmaipada,
        0,
        "hinuyAt",
        "8.4.56",
    ),
    ("05.0032", "laN", Pada::Parasmaipada, 0, "ariRot", "8.4.56"),
    (
        "05.0032",
        "viDiliN",
        Pada::Parasmaipada,
        0,
        "riRuyAt",
        "8.4.56",
    ),
    ("01.0001", "loT", Pada::Parasmaipada, 0, "BavatAd", "7.1.35"),
    (
        "01.0001",
        "loT",
        Pada::Parasmaipada,
        0,
        "BavatAt",
        "7.1.35+8.4.56",
    ),
    ("01.0001", "loT", Pada::Parasmaipada, 3, "BavatAd", "7.1.35"),
    (
        "01.0001",
        "loT",
        Pada::Parasmaipada,
        3,
        "BavatAt",
        "7.1.35+8.4.56",
    ),
    ("01.1049", "loT", Pada::Parasmaipada, 0, "nayatAd", "7.1.35"),
    (
        "01.1049",
        "loT",
        Pada::Parasmaipada,
        0,
        "nayatAt",
        "7.1.35+8.4.56",
    ),
    ("01.1049", "loT", Pada::Parasmaipada, 3, "nayatAd", "7.1.35"),
    (
        "01.1049",
        "loT",
        Pada::Parasmaipada,
        3,
        "nayatAt",
        "7.1.35+8.4.56",
    ),
    ("01.0642", "loT", Pada::Parasmaipada, 0, "jayatAd", "7.1.35"),
    (
        "01.0642",
        "loT",
        Pada::Parasmaipada,
        0,
        "jayatAt",
        "7.1.35+8.4.56",
    ),
    ("01.0642", "loT", Pada::Parasmaipada, 3, "jayatAd", "7.1.35"),
    (
        "01.0642",
        "loT",
        Pada::Parasmaipada,
        3,
        "jayatAt",
        "7.1.35+8.4.56",
    ),
    (
        "01.1082",
        "loT",
        Pada::Parasmaipada,
        0,
        "smaratAd",
        "7.1.35",
    ),
    (
        "01.1082",
        "loT",
        Pada::Parasmaipada,
        0,
        "smaratAt",
        "7.1.35+8.4.56",
    ),
    (
        "01.1082",
        "loT",
        Pada::Parasmaipada,
        3,
        "smaratAd",
        "7.1.35",
    ),
    (
        "01.1082",
        "loT",
        Pada::Parasmaipada,
        3,
        "smaratAt",
        "7.1.35+8.4.56",
    ),
    ("01.0381", "loT", Pada::Parasmaipada, 0, "paWatAd", "7.1.35"),
    (
        "01.0381",
        "loT",
        Pada::Parasmaipada,
        0,
        "paWatAt",
        "7.1.35+8.4.56",
    ),
    ("01.0381", "loT", Pada::Parasmaipada, 3, "paWatAd", "7.1.35"),
    (
        "01.0381",
        "loT",
        Pada::Parasmaipada,
        3,
        "paWatAt",
        "7.1.35+8.4.56",
    ),
    ("01.1164", "loT", Pada::Parasmaipada, 0, "vadatAd", "7.1.35"),
    (
        "01.1164",
        "loT",
        Pada::Parasmaipada,
        0,
        "vadatAt",
        "7.1.35+8.4.56",
    ),
    ("01.1164", "loT", Pada::Parasmaipada, 3, "vadatAd", "7.1.35"),
    (
        "01.1164",
        "loT",
        Pada::Parasmaipada,
        3,
        "vadatAt",
        "7.1.35+8.4.56",
    ),
    (
        "04.0001",
        "loT",
        Pada::Parasmaipada,
        0,
        "dIvyatAd",
        "7.1.35",
    ),
    (
        "04.0001",
        "loT",
        Pada::Parasmaipada,
        0,
        "dIvyatAt",
        "7.1.35+8.4.56",
    ),
    (
        "04.0001",
        "loT",
        Pada::Parasmaipada,
        3,
        "dIvyatAd",
        "7.1.35",
    ),
    (
        "04.0001",
        "loT",
        Pada::Parasmaipada,
        3,
        "dIvyatAt",
        "7.1.35+8.4.56",
    ),
    (
        "04.0091",
        "loT",
        Pada::Parasmaipada,
        0,
        "naSyatAd",
        "7.1.35",
    ),
    (
        "04.0091",
        "loT",
        Pada::Parasmaipada,
        0,
        "naSyatAt",
        "7.1.35+8.4.56",
    ),
    (
        "04.0091",
        "loT",
        Pada::Parasmaipada,
        3,
        "naSyatAd",
        "7.1.35",
    ),
    (
        "04.0091",
        "loT",
        Pada::Parasmaipada,
        3,
        "naSyatAt",
        "7.1.35+8.4.56",
    ),
    (
        "04.0146",
        "loT",
        Pada::Parasmaipada,
        0,
        "kupyatAd",
        "7.1.35",
    ),
    (
        "04.0146",
        "loT",
        Pada::Parasmaipada,
        0,
        "kupyatAt",
        "7.1.35+8.4.56",
    ),
    (
        "04.0146",
        "loT",
        Pada::Parasmaipada,
        3,
        "kupyatAd",
        "7.1.35",
    ),
    (
        "04.0146",
        "loT",
        Pada::Parasmaipada,
        3,
        "kupyatAt",
        "7.1.35+8.4.56",
    ),
    ("06.0001", "loT", Pada::Parasmaipada, 0, "tudatAd", "7.1.35"),
    (
        "06.0001",
        "loT",
        Pada::Parasmaipada,
        0,
        "tudatAt",
        "7.1.35+8.4.56",
    ),
    ("06.0001", "loT", Pada::Parasmaipada, 3, "tudatAd", "7.1.35"),
    (
        "06.0001",
        "loT",
        Pada::Parasmaipada,
        3,
        "tudatAt",
        "7.1.35+8.4.56",
    ),
    ("06.0092", "loT", Pada::Parasmaipada, 0, "liKatAd", "7.1.35"),
    (
        "06.0092",
        "loT",
        Pada::Parasmaipada,
        0,
        "liKatAt",
        "7.1.35+8.4.56",
    ),
    ("06.0092", "loT", Pada::Parasmaipada, 3, "liKatAd", "7.1.35"),
    (
        "06.0092",
        "loT",
        Pada::Parasmaipada,
        3,
        "liKatAt",
        "7.1.35+8.4.56",
    ),
    ("06.0160", "loT", Pada::Parasmaipada, 0, "viSatAd", "7.1.35"),
    (
        "06.0160",
        "loT",
        Pada::Parasmaipada,
        0,
        "viSatAt",
        "7.1.35+8.4.56",
    ),
    ("06.0160", "loT", Pada::Parasmaipada, 3, "viSatAd", "7.1.35"),
    (
        "06.0160",
        "loT",
        Pada::Parasmaipada,
        3,
        "viSatAt",
        "7.1.35+8.4.56",
    ),
    ("02.0044", "loT", Pada::Parasmaipada, 0, "yAtAd", "7.1.35"),
    (
        "02.0044",
        "loT",
        Pada::Parasmaipada,
        0,
        "yAtAt",
        "7.1.35+8.4.56",
    ),
    ("02.0044", "loT", Pada::Parasmaipada, 3, "yAtAd", "7.1.35"),
    (
        "02.0044",
        "loT",
        Pada::Parasmaipada,
        3,
        "yAtAt",
        "7.1.35+8.4.56",
    ),
    ("02.0045", "loT", Pada::Parasmaipada, 0, "vAtAd", "7.1.35"),
    (
        "02.0045",
        "loT",
        Pada::Parasmaipada,
        0,
        "vAtAt",
        "7.1.35+8.4.56",
    ),
    ("02.0045", "loT", Pada::Parasmaipada, 3, "vAtAd", "7.1.35"),
    (
        "02.0045",
        "loT",
        Pada::Parasmaipada,
        3,
        "vAtAt",
        "7.1.35+8.4.56",
    ),
    ("02.0001", "loT", Pada::Parasmaipada, 0, "attAd", "7.1.35"),
    (
        "02.0001",
        "loT",
        Pada::Parasmaipada,
        0,
        "attAt",
        "7.1.35+8.4.56",
    ),
    ("02.0001", "loT", Pada::Parasmaipada, 3, "attAd", "7.1.35"),
    (
        "02.0001",
        "loT",
        Pada::Parasmaipada,
        3,
        "attAt",
        "7.1.35+8.4.56",
    ),
    (
        "09.0058",
        "loT",
        Pada::Parasmaipada,
        0,
        "kliSnItAd",
        "7.1.35",
    ),
    (
        "09.0058",
        "loT",
        Pada::Parasmaipada,
        0,
        "kliSnItAt",
        "7.1.35+8.4.56",
    ),
    (
        "09.0058",
        "loT",
        Pada::Parasmaipada,
        3,
        "kliSnItAd",
        "7.1.35",
    ),
    (
        "09.0058",
        "loT",
        Pada::Parasmaipada,
        3,
        "kliSnItAt",
        "7.1.35+8.4.56",
    ),
    (
        "09.0053",
        "loT",
        Pada::Parasmaipada,
        0,
        "guDnItAd",
        "7.1.35",
    ),
    (
        "09.0053",
        "loT",
        Pada::Parasmaipada,
        0,
        "guDnItAt",
        "7.1.35+8.4.56",
    ),
    (
        "09.0053",
        "loT",
        Pada::Parasmaipada,
        3,
        "guDnItAd",
        "7.1.35",
    ),
    (
        "09.0053",
        "loT",
        Pada::Parasmaipada,
        3,
        "guDnItAt",
        "7.1.35+8.4.56",
    ),
    ("09.0059", "loT", Pada::Parasmaipada, 0, "aSnItAd", "7.1.35"),
    (
        "09.0059",
        "loT",
        Pada::Parasmaipada,
        0,
        "aSnItAt",
        "7.1.35+8.4.56",
    ),
    ("09.0059", "loT", Pada::Parasmaipada, 3, "aSnItAd", "7.1.35"),
    (
        "09.0059",
        "loT",
        Pada::Parasmaipada,
        3,
        "aSnItAt",
        "7.1.35+8.4.56",
    ),
    (
        "09.0066",
        "loT",
        Pada::Parasmaipada,
        0,
        "muzRItAd",
        "7.1.35",
    ),
    (
        "09.0066",
        "loT",
        Pada::Parasmaipada,
        0,
        "muzRItAt",
        "7.1.35+8.4.56",
    ),
    (
        "09.0066",
        "loT",
        Pada::Parasmaipada,
        3,
        "muzRItAd",
        "7.1.35",
    ),
    (
        "09.0066",
        "loT",
        Pada::Parasmaipada,
        3,
        "muzRItAt",
        "7.1.35+8.4.56",
    ),
    (
        "09.0040",
        "loT",
        Pada::Parasmaipada,
        0,
        "vrIRItAd",
        "7.1.35",
    ),
    (
        "09.0040",
        "loT",
        Pada::Parasmaipada,
        0,
        "vrIRItAt",
        "7.1.35+8.4.56",
    ),
    (
        "09.0040",
        "loT",
        Pada::Parasmaipada,
        3,
        "vrIRItAd",
        "7.1.35",
    ),
    (
        "09.0040",
        "loT",
        Pada::Parasmaipada,
        3,
        "vrIRItAt",
        "7.1.35+8.4.56",
    ),
    ("05.0016", "loT", Pada::Parasmaipada, 0, "ApnutAd", "7.1.35"),
    (
        "05.0016",
        "loT",
        Pada::Parasmaipada,
        0,
        "ApnutAt",
        "7.1.35+8.4.56",
    ),
    ("05.0016", "loT", Pada::Parasmaipada, 3, "ApnutAd", "7.1.35"),
    (
        "05.0016",
        "loT",
        Pada::Parasmaipada,
        3,
        "ApnutAt",
        "7.1.35+8.4.56",
    ),
    (
        "05.0017",
        "loT",
        Pada::Parasmaipada,
        0,
        "SaknutAd",
        "7.1.35",
    ),
    (
        "05.0017",
        "loT",
        Pada::Parasmaipada,
        0,
        "SaknutAt",
        "7.1.35+8.4.56",
    ),
    (
        "05.0017",
        "loT",
        Pada::Parasmaipada,
        3,
        "SaknutAd",
        "7.1.35",
    ),
    (
        "05.0017",
        "loT",
        Pada::Parasmaipada,
        3,
        "SaknutAt",
        "7.1.35+8.4.56",
    ),
    ("05.0012", "loT", Pada::Parasmaipada, 0, "hinutAd", "7.1.35"),
    (
        "05.0012",
        "loT",
        Pada::Parasmaipada,
        0,
        "hinutAt",
        "7.1.35+8.4.56",
    ),
    ("05.0012", "loT", Pada::Parasmaipada, 3, "hinutAd", "7.1.35"),
    (
        "05.0012",
        "loT",
        Pada::Parasmaipada,
        3,
        "hinutAt",
        "7.1.35+8.4.56",
    ),
    ("05.0032", "loT", Pada::Parasmaipada, 0, "riRutAd", "7.1.35"),
    (
        "05.0032",
        "loT",
        Pada::Parasmaipada,
        0,
        "riRutAt",
        "7.1.35+8.4.56",
    ),
    ("05.0032", "loT", Pada::Parasmaipada, 3, "riRutAd", "7.1.35"),
    (
        "05.0032",
        "loT",
        Pada::Parasmaipada,
        3,
        "riRutAt",
        "7.1.35+8.4.56",
    ),
    ("02.0044", "laN", Pada::Parasmaipada, 2, "ayuH", "3.4.111"),
    ("02.0045", "laN", Pada::Parasmaipada, 2, "avuH", "3.4.111"),
    ("07.0010", "laT", Pada::Parasmaipada, 1, "kfntaH", "8.4.65"),
    ("07.0010", "laT", Pada::Parasmaipada, 4, "kfnTaH", "8.4.65"),
    ("07.0010", "laT", Pada::Parasmaipada, 5, "kfnTa", "8.4.65"),
    ("07.0010", "laN", Pada::Parasmaipada, 0, "akfRat", "8.4.56"),
    ("07.0010", "laN", Pada::Parasmaipada, 1, "akfntAm", "8.4.65"),
    ("07.0010", "laN", Pada::Parasmaipada, 3, "akfRat", "8.4.56"),
    ("07.0010", "laN", Pada::Parasmaipada, 3, "akfRaH", "8.2.75"),
    ("07.0010", "laN", Pada::Parasmaipada, 4, "akfntam", "8.4.65"),
    ("07.0010", "laN", Pada::Parasmaipada, 5, "akfnta", "8.4.65"),
    ("07.0010", "loT", Pada::Parasmaipada, 0, "kfnttAd", "7.1.35"),
    (
        "07.0010",
        "loT",
        Pada::Parasmaipada,
        0,
        "kfntAd",
        "7.1.35+8.4.65",
    ),
    (
        "07.0010",
        "loT",
        Pada::Parasmaipada,
        0,
        "kfnttAt",
        "7.1.35+8.4.56",
    ),
    (
        "07.0010",
        "loT",
        Pada::Parasmaipada,
        0,
        "kfntAt",
        "7.1.35+8.4.65+8.4.56",
    ),
    ("07.0010", "loT", Pada::Parasmaipada, 1, "kfntAm", "8.4.65"),
    ("07.0010", "loT", Pada::Parasmaipada, 3, "kfnDi", "8.4.65"),
    ("07.0010", "loT", Pada::Parasmaipada, 3, "kfnttAd", "7.1.35"),
    (
        "07.0010",
        "loT",
        Pada::Parasmaipada,
        3,
        "kfntAd",
        "7.1.35+8.4.65",
    ),
    (
        "07.0010",
        "loT",
        Pada::Parasmaipada,
        3,
        "kfnttAt",
        "7.1.35+8.4.56",
    ),
    (
        "07.0010",
        "loT",
        Pada::Parasmaipada,
        3,
        "kfntAt",
        "7.1.35+8.4.65+8.4.56",
    ),
    ("07.0010", "loT", Pada::Parasmaipada, 4, "kfntam", "8.4.65"),
    ("07.0010", "loT", Pada::Parasmaipada, 5, "kfnta", "8.4.65"),
    (
        "07.0010",
        "viDiliN",
        Pada::Parasmaipada,
        0,
        "kfntyAt",
        "8.4.56",
    ),
    ("07.0019", "laN", Pada::Parasmaipada, 0, "ahinat", "8.4.56"),
    ("07.0019", "laN", Pada::Parasmaipada, 3, "ahinat", "8.4.56"),
    ("07.0019", "laN", Pada::Parasmaipada, 3, "ahinaH", "8.2.74"),
    ("07.0019", "loT", Pada::Parasmaipada, 0, "hiMstAd", "7.1.35"),
    (
        "07.0019",
        "loT",
        Pada::Parasmaipada,
        0,
        "hiMstAt",
        "7.1.35+8.4.56",
    ),
    ("07.0019", "loT", Pada::Parasmaipada, 3, "hiMstAd", "7.1.35"),
    (
        "07.0019",
        "loT",
        Pada::Parasmaipada,
        3,
        "hiMstAt",
        "7.1.35+8.4.56",
    ),
    (
        "07.0019",
        "viDiliN",
        Pada::Parasmaipada,
        0,
        "hiMsyAt",
        "8.4.56",
    ),
    ("07.0012", "laT", Pada::Atmanepada, 0, "Kinte", "8.4.65"),
    ("07.0012", "laT", Pada::Atmanepada, 5, "KinDve", "8.4.65"),
    ("07.0012", "laN", Pada::Atmanepada, 0, "aKinta", "8.4.65"),
    ("07.0012", "laN", Pada::Atmanepada, 3, "aKinTAH", "8.4.65"),
    ("07.0012", "laN", Pada::Atmanepada, 5, "aKinDvam", "8.4.65"),
    ("07.0012", "loT", Pada::Atmanepada, 0, "KintAm", "8.4.65"),
    ("07.0012", "loT", Pada::Atmanepada, 5, "KinDvam", "8.4.65"),
    ("07.0016", "laN", Pada::Parasmaipada, 0, "aBanak", "8.4.56"),
    ("07.0016", "laN", Pada::Parasmaipada, 3, "aBanak", "8.4.56"),
    ("07.0016", "loT", Pada::Parasmaipada, 0, "BaNktAd", "7.1.35"),
    (
        "07.0016",
        "loT",
        Pada::Parasmaipada,
        0,
        "BaNktAt",
        "7.1.35+8.4.56",
    ),
    ("07.0016", "loT", Pada::Parasmaipada, 3, "BaNktAd", "7.1.35"),
    (
        "07.0016",
        "loT",
        Pada::Parasmaipada,
        3,
        "BaNktAt",
        "7.1.35+8.4.56",
    ),
    (
        "07.0016",
        "viDiliN",
        Pada::Parasmaipada,
        0,
        "BaYjyAt",
        "8.4.56",
    ),
    ("07.0015", "laN", Pada::Parasmaipada, 0, "apinaw", "8.4.56"),
    ("07.0015", "laN", Pada::Parasmaipada, 3, "apinaw", "8.4.56"),
    ("07.0015", "loT", Pada::Parasmaipada, 0, "piMzwAd", "7.1.35"),
    (
        "07.0015",
        "loT",
        Pada::Parasmaipada,
        0,
        "piMzwAt",
        "7.1.35+8.4.56",
    ),
    ("07.0015", "loT", Pada::Parasmaipada, 3, "piRQi", "8.4.65"),
    ("07.0015", "loT", Pada::Parasmaipada, 3, "piMzwAd", "7.1.35"),
    (
        "07.0015",
        "loT",
        Pada::Parasmaipada,
        3,
        "piMzwAt",
        "7.1.35+8.4.56",
    ),
    (
        "07.0015",
        "viDiliN",
        Pada::Parasmaipada,
        0,
        "piMzyAt",
        "8.4.56",
    ),
    ("07.0011", "laT", Pada::Atmanepada, 0, "inDe", "8.4.65"),
    ("07.0011", "laT", Pada::Atmanepada, 5, "inDve", "8.4.65"),
    ("07.0011", "laN", Pada::Atmanepada, 0, "EnDa", "8.4.65"),
    ("07.0011", "laN", Pada::Atmanepada, 3, "EnDAH", "8.4.65"),
    ("07.0011", "laN", Pada::Atmanepada, 5, "EnDvam", "8.4.65"),
    ("07.0011", "loT", Pada::Atmanepada, 0, "inDAm", "8.4.65"),
    ("07.0011", "loT", Pada::Atmanepada, 5, "inDvam", "8.4.65"),
    ("07.0001", "laT", Pada::Parasmaipada, 1, "runDaH", "8.4.65"),
    ("07.0001", "laT", Pada::Parasmaipada, 4, "runDaH", "8.4.65"),
    ("07.0001", "laT", Pada::Parasmaipada, 5, "runDa", "8.4.65"),
    ("07.0001", "laN", Pada::Parasmaipada, 0, "aruRat", "8.4.56"),
    ("07.0001", "laN", Pada::Parasmaipada, 1, "arunDAm", "8.4.65"),
    ("07.0001", "laN", Pada::Parasmaipada, 3, "aruRat", "8.4.56"),
    ("07.0001", "laN", Pada::Parasmaipada, 3, "aruRaH", "8.2.75"),
    ("07.0001", "laN", Pada::Parasmaipada, 4, "arunDam", "8.4.65"),
    ("07.0001", "laN", Pada::Parasmaipada, 5, "arunDa", "8.4.65"),
    ("07.0001", "loT", Pada::Parasmaipada, 0, "rundDAd", "7.1.35"),
    (
        "07.0001",
        "loT",
        Pada::Parasmaipada,
        0,
        "runDAd",
        "7.1.35+8.4.65",
    ),
    (
        "07.0001",
        "loT",
        Pada::Parasmaipada,
        0,
        "rundDAt",
        "7.1.35+8.4.56",
    ),
    (
        "07.0001",
        "loT",
        Pada::Parasmaipada,
        0,
        "runDAt",
        "7.1.35+8.4.65+8.4.56",
    ),
    ("07.0001", "loT", Pada::Parasmaipada, 1, "runDAm", "8.4.65"),
    ("07.0001", "loT", Pada::Parasmaipada, 3, "runDi", "8.4.65"),
    ("07.0001", "loT", Pada::Parasmaipada, 3, "rundDAd", "7.1.35"),
    (
        "07.0001",
        "loT",
        Pada::Parasmaipada,
        3,
        "runDAd",
        "7.1.35+8.4.65",
    ),
    (
        "07.0001",
        "loT",
        Pada::Parasmaipada,
        3,
        "rundDAt",
        "7.1.35+8.4.56",
    ),
    (
        "07.0001",
        "loT",
        Pada::Parasmaipada,
        3,
        "runDAt",
        "7.1.35+8.4.65+8.4.56",
    ),
    ("07.0001", "loT", Pada::Parasmaipada, 4, "runDam", "8.4.65"),
    ("07.0001", "loT", Pada::Parasmaipada, 5, "runDa", "8.4.65"),
    (
        "07.0001",
        "viDiliN",
        Pada::Parasmaipada,
        0,
        "runDyAt",
        "8.4.56",
    ),
    ("07.0001", "laT", Pada::Atmanepada, 0, "runDe", "8.4.65"),
    ("07.0001", "laT", Pada::Atmanepada, 5, "runDve", "8.4.65"),
    ("07.0001", "laN", Pada::Atmanepada, 0, "arunDa", "8.4.65"),
    ("07.0001", "laN", Pada::Atmanepada, 3, "arunDAH", "8.4.65"),
    ("07.0001", "laN", Pada::Atmanepada, 5, "arunDvam", "8.4.65"),
    ("07.0001", "loT", Pada::Atmanepada, 0, "runDAm", "8.4.65"),
    ("07.0001", "loT", Pada::Atmanepada, 5, "runDvam", "8.4.65"),
    ("07.0002", "laT", Pada::Parasmaipada, 1, "BintaH", "8.4.65"),
    ("07.0002", "laT", Pada::Parasmaipada, 4, "BinTaH", "8.4.65"),
    ("07.0002", "laT", Pada::Parasmaipada, 5, "BinTa", "8.4.65"),
    ("07.0002", "laN", Pada::Parasmaipada, 0, "aBinat", "8.4.56"),
    ("07.0002", "laN", Pada::Parasmaipada, 1, "aBintAm", "8.4.65"),
    ("07.0002", "laN", Pada::Parasmaipada, 3, "aBinat", "8.4.56"),
    ("07.0002", "laN", Pada::Parasmaipada, 3, "aBinaH", "8.2.75"),
    ("07.0002", "laN", Pada::Parasmaipada, 4, "aBintam", "8.4.65"),
    ("07.0002", "laN", Pada::Parasmaipada, 5, "aBinta", "8.4.65"),
    ("07.0002", "loT", Pada::Parasmaipada, 0, "BinttAd", "7.1.35"),
    (
        "07.0002",
        "loT",
        Pada::Parasmaipada,
        0,
        "BinttAt",
        "7.1.35+8.4.56",
    ),
    (
        "07.0002",
        "loT",
        Pada::Parasmaipada,
        0,
        "BintAd",
        "7.1.35+8.4.65",
    ),
    (
        "07.0002",
        "loT",
        Pada::Parasmaipada,
        0,
        "BintAt",
        "7.1.35+8.4.65+8.4.56",
    ),
    ("07.0002", "loT", Pada::Parasmaipada, 1, "BintAm", "8.4.65"),
    ("07.0002", "loT", Pada::Parasmaipada, 3, "BinDi", "8.4.65"),
    ("07.0002", "loT", Pada::Parasmaipada, 3, "BinttAd", "7.1.35"),
    (
        "07.0002",
        "loT",
        Pada::Parasmaipada,
        3,
        "BinttAt",
        "7.1.35+8.4.56",
    ),
    (
        "07.0002",
        "loT",
        Pada::Parasmaipada,
        3,
        "BintAd",
        "7.1.35+8.4.65",
    ),
    (
        "07.0002",
        "loT",
        Pada::Parasmaipada,
        3,
        "BintAt",
        "7.1.35+8.4.65+8.4.56",
    ),
    ("07.0002", "loT", Pada::Parasmaipada, 4, "Bintam", "8.4.65"),
    ("07.0002", "loT", Pada::Parasmaipada, 5, "Binta", "8.4.65"),
    (
        "07.0002",
        "viDiliN",
        Pada::Parasmaipada,
        0,
        "BindyAt",
        "8.4.56",
    ),
    ("07.0002", "laT", Pada::Atmanepada, 0, "Binte", "8.4.65"),
    ("07.0002", "laT", Pada::Atmanepada, 5, "BinDve", "8.4.65"),
    ("07.0002", "laN", Pada::Atmanepada, 0, "aBinta", "8.4.65"),
    ("07.0002", "laN", Pada::Atmanepada, 3, "aBinTAH", "8.4.65"),
    ("07.0002", "laN", Pada::Atmanepada, 5, "aBinDvam", "8.4.65"),
    ("07.0002", "loT", Pada::Atmanepada, 0, "BintAm", "8.4.65"),
    ("07.0002", "loT", Pada::Atmanepada, 5, "BinDvam", "8.4.65"),
    ("07.0006", "laT", Pada::Parasmaipada, 1, "kzuntaH", "8.4.65"),
    ("07.0006", "laT", Pada::Parasmaipada, 4, "kzunTaH", "8.4.65"),
    ("07.0006", "laT", Pada::Parasmaipada, 5, "kzunTa", "8.4.65"),
    ("07.0006", "laN", Pada::Parasmaipada, 0, "akzuRat", "8.4.56"),
    (
        "07.0006",
        "laN",
        Pada::Parasmaipada,
        1,
        "akzuntAm",
        "8.4.65",
    ),
    ("07.0006", "laN", Pada::Parasmaipada, 3, "akzuRat", "8.4.56"),
    ("07.0006", "laN", Pada::Parasmaipada, 3, "akzuRaH", "8.2.75"),
    (
        "07.0006",
        "laN",
        Pada::Parasmaipada,
        4,
        "akzuntam",
        "8.4.65",
    ),
    ("07.0006", "laN", Pada::Parasmaipada, 5, "akzunta", "8.4.65"),
    (
        "07.0006",
        "loT",
        Pada::Parasmaipada,
        0,
        "kzunttAd",
        "7.1.35",
    ),
    (
        "07.0006",
        "loT",
        Pada::Parasmaipada,
        0,
        "kzunttAt",
        "7.1.35+8.4.56",
    ),
    (
        "07.0006",
        "loT",
        Pada::Parasmaipada,
        0,
        "kzuntAd",
        "7.1.35+8.4.65",
    ),
    (
        "07.0006",
        "loT",
        Pada::Parasmaipada,
        0,
        "kzuntAt",
        "7.1.35+8.4.65+8.4.56",
    ),
    ("07.0006", "loT", Pada::Parasmaipada, 1, "kzuntAm", "8.4.65"),
    ("07.0006", "loT", Pada::Parasmaipada, 3, "kzunDi", "8.4.65"),
    (
        "07.0006",
        "loT",
        Pada::Parasmaipada,
        3,
        "kzunttAd",
        "7.1.35",
    ),
    (
        "07.0006",
        "loT",
        Pada::Parasmaipada,
        3,
        "kzunttAt",
        "7.1.35+8.4.56",
    ),
    (
        "07.0006",
        "loT",
        Pada::Parasmaipada,
        3,
        "kzuntAd",
        "7.1.35+8.4.65",
    ),
    (
        "07.0006",
        "loT",
        Pada::Parasmaipada,
        3,
        "kzuntAt",
        "7.1.35+8.4.65+8.4.56",
    ),
    ("07.0006", "loT", Pada::Parasmaipada, 4, "kzuntam", "8.4.65"),
    ("07.0006", "loT", Pada::Parasmaipada, 5, "kzunta", "8.4.65"),
    (
        "07.0006",
        "viDiliN",
        Pada::Parasmaipada,
        0,
        "kzundyAt",
        "8.4.56",
    ),
    ("07.0006", "laT", Pada::Atmanepada, 0, "kzunte", "8.4.65"),
    ("07.0006", "laT", Pada::Atmanepada, 5, "kzunDve", "8.4.65"),
    ("07.0006", "laN", Pada::Atmanepada, 0, "akzunta", "8.4.65"),
    ("07.0006", "laN", Pada::Atmanepada, 3, "akzunTAH", "8.4.65"),
    ("07.0006", "laN", Pada::Atmanepada, 5, "akzunDvam", "8.4.65"),
    ("07.0006", "loT", Pada::Atmanepada, 0, "kzuntAm", "8.4.65"),
    ("07.0006", "loT", Pada::Atmanepada, 5, "kzunDvam", "8.4.65"),
    ("07.0007", "laN", Pada::Parasmaipada, 0, "ayunak", "8.4.56"),
    ("07.0007", "laN", Pada::Parasmaipada, 3, "ayunak", "8.4.56"),
    ("07.0007", "loT", Pada::Parasmaipada, 0, "yuNktAd", "7.1.35"),
    (
        "07.0007",
        "loT",
        Pada::Parasmaipada,
        0,
        "yuNktAt",
        "7.1.35+8.4.56",
    ),
    ("07.0007", "loT", Pada::Parasmaipada, 3, "yuNktAd", "7.1.35"),
    (
        "07.0007",
        "loT",
        Pada::Parasmaipada,
        3,
        "yuNktAt",
        "7.1.35+8.4.56",
    ),
    (
        "07.0007",
        "viDiliN",
        Pada::Parasmaipada,
        0,
        "yuYjyAt",
        "8.4.56",
    ),
    ("07.0009", "laT", Pada::Parasmaipada, 1, "tfntaH", "8.4.65"),
    ("07.0009", "laT", Pada::Parasmaipada, 4, "tfnTaH", "8.4.65"),
    ("07.0009", "laT", Pada::Parasmaipada, 5, "tfnTa", "8.4.65"),
    ("07.0009", "laN", Pada::Parasmaipada, 0, "atfRat", "8.4.56"),
    ("07.0009", "laN", Pada::Parasmaipada, 1, "atfntAm", "8.4.65"),
    ("07.0009", "laN", Pada::Parasmaipada, 3, "atfRat", "8.4.56"),
    ("07.0009", "laN", Pada::Parasmaipada, 3, "atfRaH", "8.2.75"),
    ("07.0009", "laN", Pada::Parasmaipada, 4, "atfntam", "8.4.65"),
    ("07.0009", "laN", Pada::Parasmaipada, 5, "atfnta", "8.4.65"),
    ("07.0009", "loT", Pada::Parasmaipada, 0, "tfnttAd", "7.1.35"),
    (
        "07.0009",
        "loT",
        Pada::Parasmaipada,
        0,
        "tfnttAt",
        "7.1.35+8.4.56",
    ),
    (
        "07.0009",
        "loT",
        Pada::Parasmaipada,
        0,
        "tfntAd",
        "7.1.35+8.4.65",
    ),
    (
        "07.0009",
        "loT",
        Pada::Parasmaipada,
        0,
        "tfntAt",
        "7.1.35+8.4.65+8.4.56",
    ),
    ("07.0009", "loT", Pada::Parasmaipada, 1, "tfntAm", "8.4.65"),
    ("07.0009", "loT", Pada::Parasmaipada, 3, "tfnDi", "8.4.65"),
    ("07.0009", "loT", Pada::Parasmaipada, 3, "tfnttAd", "7.1.35"),
    (
        "07.0009",
        "loT",
        Pada::Parasmaipada,
        3,
        "tfnttAt",
        "7.1.35+8.4.56",
    ),
    (
        "07.0009",
        "loT",
        Pada::Parasmaipada,
        3,
        "tfntAd",
        "7.1.35+8.4.65",
    ),
    (
        "07.0009",
        "loT",
        Pada::Parasmaipada,
        3,
        "tfntAt",
        "7.1.35+8.4.65+8.4.56",
    ),
    ("07.0009", "loT", Pada::Parasmaipada, 4, "tfntam", "8.4.65"),
    ("07.0009", "loT", Pada::Parasmaipada, 5, "tfnta", "8.4.65"),
    (
        "07.0009",
        "viDiliN",
        Pada::Parasmaipada,
        0,
        "tfndyAt",
        "8.4.56",
    ),
    ("07.0009", "laT", Pada::Atmanepada, 0, "tfnte", "8.4.65"),
    ("07.0009", "laT", Pada::Atmanepada, 5, "tfnDve", "8.4.65"),
    ("07.0009", "laN", Pada::Atmanepada, 0, "atfnta", "8.4.65"),
    ("07.0009", "laN", Pada::Atmanepada, 3, "atfnTAH", "8.4.65"),
    ("07.0009", "laN", Pada::Atmanepada, 5, "atfnDvam", "8.4.65"),
    ("07.0009", "loT", Pada::Atmanepada, 0, "tfntAm", "8.4.65"),
    ("07.0009", "loT", Pada::Atmanepada, 5, "tfnDvam", "8.4.65"),
    ("07.0004", "laN", Pada::Parasmaipada, 0, "ariRak", "8.4.56"),
    ("07.0004", "laN", Pada::Parasmaipada, 3, "ariRak", "8.4.56"),
    ("07.0004", "loT", Pada::Parasmaipada, 0, "riNktAd", "7.1.35"),
    (
        "07.0004",
        "loT",
        Pada::Parasmaipada,
        0,
        "riNktAt",
        "7.1.35+8.4.56",
    ),
    ("07.0004", "loT", Pada::Parasmaipada, 3, "riNktAd", "7.1.35"),
    (
        "07.0004",
        "loT",
        Pada::Parasmaipada,
        3,
        "riNktAt",
        "7.1.35+8.4.56",
    ),
    (
        "07.0004",
        "viDiliN",
        Pada::Parasmaipada,
        0,
        "riYcyAt",
        "8.4.56",
    ),
    ("07.0005", "laN", Pada::Parasmaipada, 0, "avinak", "8.4.56"),
    ("07.0005", "laN", Pada::Parasmaipada, 3, "avinak", "8.4.56"),
    ("07.0005", "loT", Pada::Parasmaipada, 0, "viNktAd", "7.1.35"),
    (
        "07.0005",
        "loT",
        Pada::Parasmaipada,
        0,
        "viNktAt",
        "7.1.35+8.4.56",
    ),
    ("07.0005", "loT", Pada::Parasmaipada, 3, "viNktAd", "7.1.35"),
    (
        "07.0005",
        "loT",
        Pada::Parasmaipada,
        3,
        "viNktAt",
        "7.1.35+8.4.56",
    ),
    (
        "07.0005",
        "viDiliN",
        Pada::Parasmaipada,
        0,
        "viYcyAt",
        "8.4.56",
    ),
];

fn lan_a_form(number: &str, pu: Purusha, va: Vacana) -> String {
    let d = dhatus().iter().find(|d| d.dhatupatha == number).unwrap();
    let branches = derive(d, Lakara::Lan, Pada::Atmanepada, pu, va);
    assert_eq!(
        branches.len(),
        1,
        "{number} laṅ ātmanepada {pu:?} {va:?} forked unexpectedly"
    );
    branches[0].text()
}

#[test]
fn labh_lan_atmanepada_all_nine_cells() {
    let expected = [
        (Purusha::Prathama, Vacana::Eka, "alaBata"),
        (Purusha::Prathama, Vacana::Dvi, "alaBetAm"),
        (Purusha::Prathama, Vacana::Bahu, "alaBanta"),
        (Purusha::Madhyama, Vacana::Eka, "alaBaTAH"),
        (Purusha::Madhyama, Vacana::Dvi, "alaBeTAm"),
        (Purusha::Madhyama, Vacana::Bahu, "alaBaDvam"),
        (Purusha::Uttama, Vacana::Eka, "alaBe"),
        (Purusha::Uttama, Vacana::Dvi, "alaBAvahi"),
        (Purusha::Uttama, Vacana::Bahu, "alaBAmahi"),
    ];
    for (pu, va, form) in expected {
        assert_eq!(lan_a_form("01.1130", pu, va), form, "{pu:?} {va:?}");
    }
}

#[test]
fn vowel_initial_roots_take_at_not_a() {
    // 6.4.72 āḍ ajādīnām (apavāda to 6.4.71) + 6.1.90 vṛddhi:
    // a+eD → ED (aidhata), a+Ikz → Ekz (aikṣata).
    assert_eq!(
        lan_a_form("01.0002", Purusha::Prathama, Vacana::Eka),
        "EData"
    );
    assert_eq!(
        lan_a_form("01.0694", Purusha::Prathama, Vacana::Eka),
        "Ekzata"
    );
}

#[test]
fn every_form_validates_and_matches() {
    let engine = Panini::new();
    for (root, lakara, row_pada, forms) in PARADIGM {
        // `PARADIGM`'s first column is a `Dhatu::dhatupatha`, but
        // `Analysis::dhatu` reports the surface `code` (deliberately not
        // unique — it's a user-facing spelling, not a key). The two must be
        // resolved against each other rather than compared directly. Because
        // both √aś rows share `code == "aS"`, matching on `code` alone would
        // let a mis-transcribed row silently bind to the WRONG root's forms
        // as long as the two roots' surfaces happen to be disjoint.
        // Comparing against `row_pada` — the row's own declared pada — rather than
        // `d.pada.padas()[0]` pins the row's claim, not the root's: it still
        // closes the √aś hole (kryādi's is parasmaipada, svādi's is
        // ātmanepada), and it is the form that also works once a root's
        // `PadaAssignment` is `Ubhayapada` and `padas()[0]` alone can no
        // longer stand in for "the pada this block is for".
        let d = dhatus().iter().find(|d| d.dhatupatha == *root).unwrap();
        for expected in forms {
            let r = engine.check(expected);
            assert!(
                matches!(r.verdict, Verdict::Valid),
                "expected VALID for {expected} ({root} {lakara})"
            );
            assert!(
                r.analyses.iter().any(|a| a.form_slp1 == *expected
                    && a.dhatu == d.code
                    && a.pada == *row_pada
                    && panini::lakara_name(a.lakara) == *lakara),
                "no {lakara} analysis of {root} produced {expected}"
            );
        }
    }
}

/// Every alternate must itself check out as a real form of the root and
/// lakāra it is filed under — same `Dhatu::dhatupatha` → `code` resolution
/// `every_form_validates_and_matches` uses, since `Analysis::dhatu` reports
/// the non-unique surface `code`. Pinned against the row's own `pada`, for
/// the same reason `every_form_validates_and_matches` is.
#[test]
fn every_alternate_validates_and_matches() {
    let engine = Panini::new();
    for (root, lakara, row_pada, _cell, form, _key) in ALTERNATES {
        let d = dhatus().iter().find(|d| d.dhatupatha == *root).unwrap();
        let r = engine.check(form);
        assert!(
            matches!(r.verdict, Verdict::Valid),
            "expected VALID for alternate {form} ({root} {lakara})"
        );
        assert!(
            r.analyses.iter().any(|a| a.form_slp1 == *form
                && a.dhatu == d.code
                && a.pada == *row_pada
                && panini::lakara_name(a.lakara) == *lakara),
            "no {lakara} analysis of {root} produced alternate {form}"
        );
    }
}

/// `derivation_set_is_exactly_pinned`'s `(r, l, p, c, _, _)` filter and
/// `every_alternate_validates_and_matches`'s `_cell` both silently ignore a
/// row whose `cell` is out of range or whose `(root, lakara, pada)` is
/// mistyped — neither assertion would ever touch the cell such a row meant
/// to name. This closes that: every `ALTERNATES` row must name a real cell
/// of a real `PARADIGM` block, pada included.
#[test]
fn every_alternate_names_a_real_cell() {
    for (root, lakara, pada, cell, form, _key) in ALTERNATES {
        assert!(
            *cell < 9,
            "alternate {form} ({root} {lakara}) has out-of-range cell {cell}"
        );
        assert!(
            PARADIGM
                .iter()
                .any(|(r, l, p, _)| r == root && l == lakara && p == pada),
            "alternate {form} names {root} {lakara} {pada:?}, which is not a PARADIGM block"
        );
    }
}

/// The optional rules, in pipeline order. Mirrors
/// `exactly_the_pinned_vikalpa_rules_are_optional` in `panini-prakriya`;
/// duplicated here rather than exported because this is an integration test
/// and the rule table is crate-internal.
const VIKALPA_RULES: &[&str] = &[
    "7.1.35", "3.4.111", "6.4.107", "8.2.74", "8.2.75", "8.4.65", "8.4.56",
];

/// `ALTERNATES` is otherwise 350 bare strings, and a string can be right for
/// the wrong reason — `BavatAt` is a real form whether or not 8.4.56 is what
/// produced it. This ties each row to the grammar: find the branch that
/// derives the row's form, intersect its log with the optional-rule set, and
/// require exactly the rules the row claims.
#[test]
fn every_alternate_names_the_vikalpa_rules_that_produced_it() {
    for (root, lakara, pada, cell, form, key) in ALTERNATES {
        let d = dhatus().iter().find(|d| d.dhatupatha == *root).unwrap();
        let (pu, va) = CELLS[*cell];
        let lak = *LAKARA_BY_NAME
            .iter()
            .find_map(|(n, l)| (n == lakara).then_some(l))
            .unwrap();
        let branch = derive(d, lak, *pada, pu, va)
            .into_iter()
            .find(|p| !p.blocked && p.text() == *form)
            .unwrap_or_else(|| panic!("no branch of {root} {lakara} cell {cell} derives {form}"));
        let applied: Vec<&str> = branch
            .log
            .iter()
            .map(|s| s.sutra.as_str())
            .filter(|s| VIKALPA_RULES.contains(s))
            .collect();
        assert_eq!(
            applied.join("+"),
            *key,
            "{form} ({root} {lakara} cell {cell})"
        );
    }
}

/// The other half of `every_form_validates_and_matches`, which only ever
/// asks "is this form derivable?" and never "what else is?". That asymmetry
/// is what lets alternates land without touching PARADIGM's strings, and it
/// is also a hole: an over-firing optional rule would fork cells nobody
/// checks. This closes it — for every cell, the set of forms the engine
/// derives must be EXACTLY its pinned form plus its pinned alternates.
#[test]
fn derivation_set_is_exactly_pinned() {
    for (root, lakara, row_pada, forms) in PARADIGM {
        let d = dhatus().iter().find(|d| d.dhatupatha == *root).unwrap();
        for (cell, expected) in forms.iter().enumerate() {
            let (pu, va) = CELLS[cell];
            let lak = *LAKARA_BY_NAME
                .iter()
                .find_map(|(n, l)| (n == lakara).then_some(l))
                .unwrap();

            let branches = derive(d, lak, *row_pada, pu, va);
            assert_eq!(
                branches[0].text(),
                *expected,
                "index 0 must be the declined derivation for {root} {lakara} cell {cell}"
            );

            let mut actual: Vec<String> = branches
                .iter()
                .filter(|p| !p.blocked)
                .map(|p| p.text())
                .collect();
            actual.sort();

            let mut want: Vec<String> = vec![(*expected).to_string()];
            want.extend(
                ALTERNATES
                    .iter()
                    .filter(|(r, l, p, c, _, _)| {
                        r == root && l == lakara && p == row_pada && *c == cell
                    })
                    .map(|(_, _, _, _, f, _)| (*f).to_string()),
            );
            want.sort();

            assert_eq!(
                actual, want,
                "derivation set for {root} {lakara} cell {cell} \
                 (pinned {expected}) is not exactly what PARADIGM + ALTERNATES say"
            );
        }
    }
}

/// Pins the shape of the derivation set the slice produces, derived from
/// `PARADIGM ∪ ALTERNATES` — the same union `derivation_set_is_exactly_pinned`
/// builds — rather than from a hand-written list. These are the numbers the
/// design-time vidyut-prakriya audit predicted for the two conventions the
/// svādi slice retired (7.1.35 tātaṅ, 8.4.56 pausal cartva), the one audited
/// divergence it resolved (3.4.111 Śākaṭāyana's jus), the three roots added
/// in rudhādi 7a (kft, his, Kid), three more added in rudhādi 7b (Banj, piz,
/// inD), and — new in the ubhayapada 1.3.72 slice — √rudh (ruD), pinned in
/// both padas, joined by the pada audit's √nī and √tud, also pinned in both
/// padas: every one of the eleven rudhādi roots forks in both loṭ and
/// laṅ, and two of them — kft and ruD — fork in all four lakāras: laṭ (kft
/// cells 1/4/5, Kid cells 0/5, inD cells 0/5, and — new in this slice — ruD
/// parasmaipada cells 1/4/5 and ātmanepada cells 0/5, all on 8.4.65), laṅ (on
/// 8.4.65, the 8.2.74/8.2.75 ru alternation, and the 8.2.23-above-8.2.41
/// śa-luk jaśtva 8.4.56 branch), loṭ (on 7.1.35/8.4.65/8.4.56, stacking up to
/// three deep, and piṣ's loṭ madhyama eka, which stacks 8.4.65 alongside
/// 7.1.35/8.4.56 four deep), and vidhiliṅ (kft/his/Banj/piz/ruD/Bid/kzud/yuj/tfd/ric/vic
/// cell 0, on 8.4.56 — Kid and inD do not fork here). Slice 7c curated four more roots —
/// √bhid (Bid), √kṣud (kzud), √yuj (yuj) and √tṛd (tfd), all four ubhayapadī
/// by 1.3.72 and pinned in both padas — and three of them join kft and ruD as
/// four-lakāra forkers: Bid, kzud and tfd each stack 7.1.35/8.4.65/8.4.56 in
/// loṭ parasmaipada exactly as kft and ruD do, while yuj forks only two deep
/// there (7.1.35/8.4.56, no 8.4.65 branch — 8.2.30 coH kuH replaces its
/// stem-final palatal `j` with the VELAR `g`, which 8.4.55 khari ca later
/// devoices to `k` before the `t` of tātaṅ, so the junction 8.4.65 would need
/// is velar-against-dental at both sites — `g`+`D` in yuNgDi, `k`+`t` in
/// yuNktAd — and never savarṇa the way the dental-final roots' `d`+`D` and
/// geminate `t`+`t` are, so 8.4.65's site never arises). The 8.2.30/8.2.39
/// generalization slice curated two more roots on exactly this shape for
/// exactly this reason — √ric (ric) and √vic (vic), each ending in a
/// palatal (`c`) rather than `j`, which 8.2.30 coH kuH — now one
/// substitution-table lookup instead of a literal `g` — substitutes with
/// the VELAR `k` rather than `g`: the same velar-against-dental mismatch
/// (`k`+`D` in riNgDi/viNgDi, `k`+`t` in riNktAd/viNktAd) keeps 8.4.65 out
/// of their loṭ parasmaipada prathama/madhyama eka too, so they join yuj
/// forking only two deep there, on 7.1.35/8.4.56. The other rule this slice
/// widened, 8.2.39 jhalāṁ jaśo'nte, now reads its own substitution table on
/// both sides instead of a `t`/`z`/`D`-only literal guard, which reaches a
/// pada-final velar for the first time: ric's and vic's laṅ prathama and
/// madhyama eka decline to `ariRag`/`avinag` (jaśtva-voiced) with 8.4.56
/// vā'vasāne supplying the optional `ariRak`/`avinak` — the same
/// √bhañj-pattern fork yuj's `ayunag`/`ayunak` already witnesses, now with
/// a second pair of roots on it:
/// 2304 cells total (256 root×lakāra blocks × 9), of which 2056 hold exactly one form,
/// 172 hold two, 65 hold three, one holds four (piṣ's loṭ madhyama eka, the
/// deepest fork added in 7b), and — the sharpest branch-count witnesses in
/// the repo, per `docs/ARCHITECTURE.md` — exactly five hold five (√kṛt's loṭ
/// prathama eka, ruD's loṭ parasmaipada prathama eka, and — new in this
/// slice — Bid's, kzud's and tfd's loṭ parasmaipada prathama eka) and five
/// hold six (√kṛt's loṭ madhyama eka, `kfndDi`/`kfnDi`'s cell, ruD's loṭ
/// parasmaipada madhyama eka, `rundDi`/`runDi`/`rundDAd`/`runDAd`/
/// `rundDAt`/`runDAt`, and — new in this slice — Bid's, kzud's and tfd's loṭ
/// parasmaipada madhyama eka, each tying √kṛt's record with the same k = 3
/// (7.1.35, 8.4.65, 8.4.56) against a 2³ bound of eight — ric and vic do not
/// join this record; per the 8.2.30/8.2.39 slice's own audit their deepest
/// cells are three forms). `ALTERNATES`
/// itself has 350 rows, keyed 81 `8.4.56`, 70 `7.1.35`, 70 `7.1.35+8.4.56`,
/// 2 `3.4.111`, 8 `6.4.107`, 93 `8.4.65`, 5 `8.2.75`, 1 `8.2.74`, 10
/// `7.1.35+8.4.65`, and 10 `7.1.35+8.4.65+8.4.56` — the assertions below are
/// complete. The audit probe that produced the original numbers ran against
/// a vidyut-prakriya checkout during design; slice 9's cross-implementation
/// audit re-ran the full check against a scratchpad vidyut-prakriya checkout
/// across all 1620 pre-7b cells with zero differences, every 7b form was
/// cross-checked the same way during that slice's design, this slice's
/// √rudh forms were audited against a vidyut-prakriya checkout at commit
/// 8da2f90 the same way, and slice 7c's four roots were audited the same way
/// against vidyut `8da2f90`, zero differences across all 2160 cells / 2496
/// forms / 53 roots, with the `entry` negative control verified failing —
/// the probe's source is committed at `tools/audit/panini_full_audit.rs`,
/// and the pada audit re-ran it over all 1872 pre-7c cells — so the numbers
/// are re-verified as well as pinned. The 8.2.30/8.2.39 generalization
/// slice's own cross-implementation audit re-ran the same probe against
/// vidyut-prakriya at commit `8da2f90` over all 2304 cells / 2654 forms / 55
/// roots with zero differences, its `entry` negative control verified
/// failing (36 √bhū cells) both times the audit was run — so these numbers
/// are re-verified as well as pinned. This test is what keeps the numbers
/// true day to day.
#[test]
fn derivation_set_shape_matches_the_audited_numbers() {
    let total_cells = PARADIGM.len() * 9;
    assert_eq!(total_cells, 2304, "256 root×lakāra blocks × 9 cells each");

    let mut ones = 0usize;
    let mut twos = 0usize;
    let mut threes = 0usize;
    let mut fours = 0usize;
    let mut fives = 0usize;
    let mut sixes = 0usize;
    for (root, lakara, row_pada, _forms) in PARADIGM {
        for cell in 0..9usize {
            let alt_count = ALTERNATES
                .iter()
                .filter(|(r, l, p, c, _, _)| {
                    r == root && l == lakara && p == row_pada && *c == cell
                })
                .count();
            match 1 + alt_count {
                1 => ones += 1,
                2 => twos += 1,
                3 => threes += 1,
                4 => fours += 1,
                5 => fives += 1,
                6 => sixes += 1,
                n => panic!("unexpected {n}-form cell in ({root}, {lakara}, {cell})"),
            }
        }
    }
    assert_eq!(ones, 2056, "one-form cells");
    assert_eq!(twos, 172, "two-form cells");
    assert_eq!(threes, 65, "three-form cells");
    assert_eq!(fours, 1, "four-form cells — piṣ's loṭ madhyama eka");
    assert_eq!(
        fives, 5,
        "five-form cells — kft loṭ prathama eka, ruD loṭ parasmaipada prathama eka, and — new \
         in this slice — Bid, kzud and tfd's loṭ parasmaipada prathama eka"
    );
    assert_eq!(
        sixes, 5,
        "six-form cells — kft loṭ madhyama eka, ruD loṭ parasmaipada madhyama eka, and — new \
         in this slice — Bid, kzud and tfd's loṭ parasmaipada madhyama eka"
    );

    assert_eq!(ALTERNATES.len(), 350, "ALTERNATES row count");
    let key_count = |key: &str| {
        ALTERNATES
            .iter()
            .filter(|(_, _, _, _, _, k)| *k == key)
            .count()
    };
    assert_eq!(key_count("8.4.56"), 81, "8.4.56-only alternates");
    assert_eq!(key_count("7.1.35"), 70, "7.1.35-only alternates");
    assert_eq!(key_count("7.1.35+8.4.56"), 70, "7.1.35+8.4.56 alternates");
    assert_eq!(key_count("3.4.111"), 2, "3.4.111 alternates");
    assert_eq!(key_count("6.4.107"), 8, "6.4.107 alternates");
    assert_eq!(key_count("8.4.65"), 93, "8.4.65-only alternates");
    assert_eq!(key_count("8.2.75"), 5, "8.2.75-only alternates");
    assert_eq!(key_count("8.2.74"), 1, "8.2.74-only alternates");
    assert_eq!(key_count("7.1.35+8.4.65"), 10, "7.1.35+8.4.65 alternates");
    assert_eq!(
        key_count("7.1.35+8.4.65+8.4.56"),
        10,
        "7.1.35+8.4.65+8.4.56 alternates"
    );
}

/// `every_form_validates_and_matches` only walks `PARADIGM`, so a root or
/// lakāra added to the enumerable space without golden rows would be checked
/// by nothing at all. This test closes that hole from the other side: every
/// (root × lakāra) pair the analyzer enumerates must either be pinned by a
/// `PARADIGM` block or appear in the explicit gated list below.
#[test]
fn paradigm_covers_every_enumerable_cell() {
    // adādi × vidhiliṅ was gated in slice 5a and ungated in slice 5b; √śī was
    // gated in slice 5f task 1 and ungated there; √nī and √tud's ātmanepada
    // blocks were gated for one commit by the pada audit, between the column
    // being corrected and the audited goldens landing. There are no gated
    // cells any more. This constant stays (empty) so the two assertions below
    // keep documenting that EVERY enumerable (root, lakara, pada) triple must
    // be pinned in PARADIGM — a future partial slice may repopulate it, but it
    // must never silently hide a missing golden block.
    const GATED: &[(&str, &str, Pada)] = &[];

    let pinned: Vec<(&str, &str, Pada)> =
        PARADIGM.iter().map(|(r, l, p, _)| (*r, *l, *p)).collect();
    let mut unpinned: Vec<(&str, &str, Pada)> = Vec::new();
    for d in dhatus() {
        for &lakara in panini_analyze::LAKARAS {
            for &pada in d.pada.padas() {
                let triple = (d.dhatupatha, panini::lakara_name(lakara), pada);
                if !pinned.contains(&triple) {
                    unpinned.push(triple);
                }
            }
        }
    }
    // `Pada` has no `Ord` of its own (`Context.pada` never needs to be
    // sorted); `pada_name` gives a stable, already-public key to sort by.
    fn sort_key<'a>(t: &(&'a str, &'a str, Pada)) -> (&'a str, &'a str, &'static str) {
        (t.0, t.1, panini::pada_name(t.2))
    }
    unpinned.sort_unstable_by_key(sort_key);
    let mut gated = GATED.to_vec();
    gated.sort_unstable_by_key(sort_key);
    assert_eq!(
        unpinned, gated,
        "every enumerable (root, lakara, pada) triple needs golden rows in PARADIGM \
         (or an explicit entry in GATED, for a cell deliberately withheld from golden coverage)"
    );
    // Catches a duplicated PARADIGM block masking a missing one above.
    let enumerable: usize = dhatus()
        .iter()
        .map(|d| d.pada.padas().len() * panini_analyze::LAKARAS.len())
        .sum();
    assert_eq!(
        PARADIGM.len() + GATED.len(),
        enumerable,
        "PARADIGM has a duplicate or stale (root, lakara, pada) block"
    );
}

#[test]
fn known_nonforms_are_invalid() {
    let engine = Panini::new();
    for bad in [
        // Real cross-lakāra confusions, not junk: laṅ endings require the
        // aṭ-āgama (6.4.71), and laṭ endings forbid it.
        "Bavat",    // laṅ 3sg ending without the augment
        "aBavanti", // augment on a laṭ form
        "aBavatu",  // augment on a loṭ form
        "aBavet",   // laṅ's aṭ-āgama on a vidhiliṅ form
        "Bavetu",   // loṭ's er uḥ ending on a vidhiliṅ stem
        // Still out of scope entirely.
        "gacCati",
        "Bavati123",
        "tiRRati",
        // Wrong pada: the root's pada assignment gates the whole derivation
        // (1.3.12 / 1.3.72 / 1.3.78) and the analyzer proposes exactly the
        // padas that assignment admits — one each for the single-pada roots
        // below, both for an ubhayapadī root like √rudh.
        "laBati", // atmanepadin root with a parasmaipada ending
        "Bavate", // parasmaipada root with an atmanepada ending
        "eDati",  // vowel-initial atmanepadin root, parasmaipada ending
        "alaBat", // laN parasmaipada shape on an atmanepadin root
        "laB",    // a bare root code is not a surface form
        // Cross-lakāra atmanepada confusions.
        "alaBeta", // laN's augment on a vidhilin form
        "laBatam", // parasmaipada dual ending on an atmanepadin root
        "laBAte",  // 7.2.81 skipped: A must become iy after the shap
        "laBesva", // lot's sva on a lat stem (3.4.91 without 3.4.90's lakara)
        "IkzAmi",  // parasmaipada uttama ending on the vowel-initial A-root
        // Wrong vikaraṇa: divādi/tudādi roots take śyan/śa, not śap, and
        // bhvādi does not take śyan.
        "divati",  // div with śap instead of śyan
        "tudyati", // tud with śyan instead of śa
        "Bavyati", // BU (bhvādi) with a śyan it has no claim to
        "naSati",  // naś with śap
        "kupati",  // kup with śap
        // Guṇa should have been blocked (1.1.5): these are the guṇa'd forms.
        "kopyati", // kup guṇa'd — 7.3.86 must be blocked by śyan's ṅit
        "todati",  // tud guṇa'd — 7.3.86 must be blocked by śa's ṅit
        "jozate",  // juṣ guṇa'd — block under ātmanepada too
        "devyati", // div guṇa'd (before 8.2.77): guṇa must be blocked
        // Wrong pada: the root's curated pada verdict gates the whole
        // derivation.
        "manyati", // atmanepadin divādi root with a parasmaipada ending
        "vidyati", // atmanepadin divādi root, parasmaipada ending
        // adādi (gaṇa 2): śap is luk'd (2.4.72). A retained-śap surface must
        // not derive, and the parasmaipada roots reject ātmanepada endings.
        "yAyati", // yā with a spurious y-śap — no derivation yields it
        "yAte",   // parasmaipada yā with an ātmanepada ending (wrong pada)
        "vAte",   // parasmaipada vā with an ātmanepada ending (wrong pada)
        "yAati",  // luk skipped: śap's `a` left standing after ā (uncoalesced)
        "yA",     // a bare root code is not a surface form
        "vA",
        // These four are the non-words the pre-5b pipeline emitted for adādi
        // vidhiliṅ before 6.1.96 / the 6.1.101 arm reduced the yāsuṭ-ā + vowel
        // junction. They stay pinned INVALID as the regression that the
        // reduction actually RAN: the real forms are yAyuH / yAyAm (and the vā
        // pair), now pinned as goldens in PARADIGM. If any of these four ever
        // validates, the junction reduction regressed.
        "yAyAuH", // 3pl: real form yāyuḥ
        "yAyAam", // 1sg: real form yāyām
        "vAyAuH",
        "vAyAam",
        "Asati",  // √ās is ātmanepada; a parasmaipada ending must not derive
        "Asante", // 3pl must be Asate (7.1.5), never the `ante` of 7.1.3
        // 8.2.25 dhi ca elides the aṅga-final `s` before Dve/Dvam. Both the
        // un-applied shape and slice 5d's jaśtva'd shape are non-words.
        "AsDve",    // s retained: the rule did not fire
        "AdDve",    // 5d's wrong form: s voiced to `d` instead of elided
        "AdDvam",   // ditto, laṅ/loṭ
        "vasDve",   // √vas, s retained
        "vadDve",   // √vas, 5d's wrong analysis
        "avasDvam", // √vas laṅ, s retained
        "vasati",   // √vas is ātmanepada; a parasmaipada ending must not derive
        // √śī (slice 5f). Each of these is a non-form the engine must never
        // produce, chosen around the slice's three new guards — but not all
        // seven are what a mutation of that guard would actually emit; see
        // the per-entry notes below where the naive reading is wrong.
        "SIte", // A genuine witness for 7.4.21's removal, not an unreachable
        // shape: 7.3.84's 1.1.5 guard now calls `following_sarvadhatuka`,
        // which on this śap-luk'd path returns the ṅit `te` ending itself
        // (there is no non-empty śap to interpose), so 1.1.5 really does
        // block 7.3.84 here. Without 7.4.21, nothing else guṇates `SI`, and
        // the surface form would be exactly `SIte`. It stays pinned INVALID
        // because 7.4.21 has not been removed; if 7.4.21 is ever dropped or
        // its own guard broken, this is the entry that would flip to VALID
        // and catch it. The rule actually responsible for the guṇa is
        // pinned independently by the ordered-trace test
        // `shete_trace_is_the_minimal_shing_guna_path` in
        // `crates/panini/tests/trace.rs`, which asserts `7.4.21` present and
        // `7.3.84` absent.
        "Sese",  // 8.3.59 not applied: ṣatva missing (real form Seze)
        "Seate", // NOT what removing 7.1.6 emits: without the ruṭ the ending
        // stays `ate`, and 6.1.78's athematic arm then fires (śap empty, `a`
        // is a vowel), emitting `Sayate` — already pinned below, which is
        // the actual witness for 7.1.6's removal.
        "SayIraran", // NOT a real derivation: dropping 7.1.6's guard against
        // firing in vidhiliṅ makes it prepend `r` to the sīyuṭ-bearing
        // ending `sIyran` (→ `rsIyran`); 7.2.79 still elides the non-final
        // `s` regardless (→ `rIyran`), but 6.1.78's athematic arm then
        // requires the ending's first character to be a vowel, and `r`
        // isn't one, so the ay-ādeśa never fires and the output diverges
        // from this string entirely. Kept pinned as a plain non-form; the
        // real form is `SayIran`.
        "Sayati", // wrong pada: an ātmanepadin root with a parasmaipada ending
        "Sayate", // the śap surviving 2.4.72 (SI + Sap + te, guṇa'd)
        "SIyate", // a divādi/tudādi-style vikaraṇa leaking into adādi
        // kryādi (gaṇa 9, slices 9a/9b). Each of these is what the slice's
        // own rule comments say would surface if the named rule misfired;
        // pinning them keeps those rules' guards honest the same way the
        // adādi and √śī groups above pin theirs.
        "kliSnIti",  // 1.2.4 misfiring on the pit ending tip (śnā stays anit)
        "kleSAna",   // 7.3.86 not blocked by 1.1.5 for śānac (guṇa'd upadhā)
        "kliSnIhi",  // 3.1.83 (śnā-lopa before hi) ordered after 6.4.113
        "vfReta",    // 6.4.112 (nA -> n) running after 6.1.87, not before
        "vfRIyta",   // 6.1.66's old is_empty() guard, silently declining for kryādi
        "vfRIsva",   // 8.3.59 before it read the preceding term instead of ANGA
        "vrIRAhi",   // 3.4.87 not tagging hi as pit
        "kliSnAyAt", // 3.4.103 not tagging yāsuṭ's ending ṅit
        // svādi (gaṇa 5). Four sūtras, three widened guards and six roots
        // landed with nothing pinned here until now; pinning them keeps
        // those rules' guards honest the same way the adādi, √śī and kryādi
        // groups above pin theirs.
        "aSnoti", // wrong pada: svādi's √aś is ātmanepada (real form aSnute);
        // also catches an id/code collapse from the other side — kryādi's
        // √aś (id "aS") is parasmaipada and DOES take this ending, so this
        // string would wrongly validate if the two "aS" rows' padas were
        // ever merged or mismatched.
        "Apnute", // wrong pada: √āp is parasmaipada (real form Apnoti)
        "ApnuDi", // 6.4.101 reading ANGA ("p", a jhal) instead of
        // sound_before_ending (śnu's "u", not a jhal) — real form Apnuhi
        "SaknuDi", // same guard, second conjunct root — real form Saknuhi
        "ApnoAni", // 6.1.78's vikaraṇa arm (svādi's third arm) removed —
        // real form ApnavAni
        "ApnuvAni", // 7.3.84's second application ordered AFTER 6.4.77/
        // 6.4.87 instead of before them — real form ApnavAni
        "hinuhi", // 6.4.106 under-firing (declining to luk hi after a
        // non-conjunct u) — real form hinu
        "Apnu", // 6.4.106 over-firing (luking hi after a conjunct u) —
        // real form Apnuhi
        "hinuvanti", // 6.4.87/6.4.77 swapped: the non-conjunct root taking
        // 6.4.77's uvaṅ instead of 6.4.87's yaṇ — real form hinvanti
        "Apnvanti", // the conjunct root taking 6.4.87's yaṇ instead of
        // 6.4.77's uvaṅ — real form Apnuvanti
        "aSnavAE", // 6.1.90's athematic arm not widened past is_empty() to
        // admit svādi's non-empty, non-a/A-final `nav` — real form aSnavE
        "henoti", // the FIRST 7.3.84 (root-relative) not blocked by śnu's
        // ṅit vikaraṇa — svādi never guṇates the root itself; real form
        // hinoti
        "reRoti", // same guard, second non-conjunct root — real form riRoti
        "kliSne", // 7.3.84's SECOND application (vikaraṇa-relative, svādi's
        // own addition) firing on kryādi's `nI` instead of declining by
        // 1.1.5 — real form kliSnAti
        // 6.4.107 over-firing. It is optional, so an over-firing guard
        // ADDS a wrong second form rather than replacing a right one —
        // invisible to any test that only asks whether the right form
        // still derives. Each pin names the guard it would breach.
        "ApnvaH",  // fired on a conjunct root — real form ApnuvaH
        "ApnmaH",  // same, bahu — real form ApnumaH
        "aSnvahe", // fired in the ātmanepada conjunct column, where no
        // svādi root is asaṁyogapūrva — real form aSnuvahe
        "hinTaH", // fired on an ending that is not m/v-initial — real
        // form hinuTaH
        "hinyAma", // `starts_with` mistaken for `contains`: vidhiliṅ's
        // yAma has an `m` but does not begin with one — real form hinuyAma
        "BavmaH", // fired where the vikaraṇa is not śnu at all, i.e. the
        // shnu_asamyogapurva guard dropped — real form BavAmaH
        // 8.2.39 jhalāṁ jaśo'nte guard pins.
        "Bavatd", // `ends_with('t')` mistaken for `contains('t')`: fires on
        // BU laṭ 3sg (which merely contains a medial `t`, not a pada-final
        // one) and blindly voices whatever the actual last character is —
        // real form Bavati
        "aBavaD", // `s.push('d')` mistaken for `s.push('D')`: the wrong jaś
        // substitute (aspirated, not the plain voiced stop the sūtra names)
        // — real form aBavad
        // 8.4.56's `is_jhal(last)` guard (Step 11 mutation 3) has since been
        // deleted outright — it was dead code, subsumed by the `cartva_of`
        // let-else right below it — so there is no longer a mutation for it
        // to pin here. 8.4.56's `vikalpa: true` -> `false` (mutation 4)
        // removes the `d`-form rather than adding a non-form, so it is
        // caught by `derivation_set_is_exactly_pinned`'s index-0 assertion,
        // not by a pin in this list.
        // 7.1.35 tātaṅ. Because the rule is optional, a broken guard ADDS a
        // wrong second form rather than replacing a right one — invisible to
        // any test that only asks whether the right form still derives.
        "ApnotAt", // 7.1.35 failing to set Ngit, so 7.3.84's second
        // (vikaraṇa-relative) application guṇates śnu — real form ApnutAt
        "kliSAnatAt", // 7.1.35 ordered AFTER 3.1.83 instead of above it, so
        // śnā had already become śāna when the ending was still `hi` — real
        // form kliSnItAt
        // 3.4.110/111 Śākaṭāyana's jus. Optional, so a broken guard adds a
        // wrong form rather than removing a right one.
        "aBavuH", // 3.4.111 losing BOTH of its second `if`'s conjuncts (the
        // ā-check and the SHAP-empty check together, not either alone —
        // dropping only the ā-check still declines on SHAP being `Bava`'s
        // live śap `a`) — real form aBavan
        "yuH", // 3.4.111 not gated to laṅ, so laṭ's yAnti forks — real
               // form yAnti
    ] {
        assert!(
            matches!(engine.check(bad).verdict, Verdict::Invalid),
            "expected INVALID for {bad}"
        );
    }
}

#[test]
fn both_ash_roots_derive() {
    let engine = Panini::new();
    for form in ["aSnute", "aSnAti"] {
        assert!(
            matches!(engine.check(form).verdict, Verdict::Valid),
            "{form}"
        );
    }
}

/// The surfaces that are genuinely pada-ambiguous — the same string pinned
/// as both a parasmaipada and an ātmanepada cell, so `check` reports two
/// analyses differing in pada. `README.md` quotes this list; before this
/// test it was hand-maintained prose with nothing behind it, and the
/// ubhayapadī root count going from three to seven in slice 7c is exactly
/// the kind of change that would have grown it silently.
///
/// `roundtrip.rs` cannot serve this purpose: it asks only whether SOME
/// analysis recovers the input, never how many there are.
#[test]
fn pada_ambiguous_surfaces_are_exactly_these() {
    let mut para: Vec<&str> = Vec::new();
    let mut atma: Vec<&str> = Vec::new();
    for (_root, _lakara, pada, forms) in PARADIGM {
        let bucket = match pada {
            Pada::Parasmaipada => &mut para,
            Pada::Atmanepada => &mut atma,
        };
        bucket.extend(forms.iter().copied());
    }

    let mut both: Vec<&str> = para.iter().copied().filter(|f| atma.contains(f)).collect();
    both.sort_unstable();
    both.dedup();

    // Measured (never hand-picked) by running this assertion against
    // `Vec::<&str>::new()` and reading the real set off the failure. The
    // pre-slice baseline (checked separately against `main`, before any
    // 7c commit) was actually ten surfaces, not the seven README.md names:
    // `rundDAm` and `arundDa` (√rudh `07.0001`, loT and laN, each ambiguous
    // against its own two padas), `anayata`/`nayatAm`/`nayetAm`/`nayeta`
    // (√nī) and `atudata`/`tudatAm`/`tudetAm`/`tudeta` (√tud) — README's
    // hand list already missed `arundDa`, `nayetAm` and `tudetAm`, and
    // spells the rudh one without its second `d`. All ten pre-slice
    // surfaces are present below, so nothing was disturbed by this slice.
    // Slice 7c's four new ubhayapadī roots contribute the other eight:
    // `BinttAm`/`aBintta` (√Bid `07.0002`), `akzuntta`/`kzunttAm`
    // (√kzud `07.0006`), `ayuNkta`/`yuNktAm` (√yuj `07.0007`), and
    // `atfntta`/`tfnttAm` (√tfd `07.0009`). The 8.2.30/8.2.39 generalization
    // slice's two new ubhayapadī roots contribute four more, the same
    // shape as yuj's pair: `ariNkta`/`riNktAm` (√ric `07.0004`) and
    // `aviNkta`/`viNktAm` (√vic `07.0005`).
    assert_eq!(
        both,
        vec![
            "BinttAm", "aBintta", "akzuntta", "anayata", "ariNkta", "arundDa", "atfntta",
            "atudata", "aviNkta", "ayuNkta", "kzunttAm", "nayatAm", "nayetAm", "nayeta", "riNktAm",
            "rundDAm", "tfnttAm", "tudatAm", "tudetAm", "tudeta", "viNktAm", "yuNktAm",
        ]
    );
}
