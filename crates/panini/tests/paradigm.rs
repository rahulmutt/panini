use panini::{Panini, Verdict};
use panini_data::{Lakara, Pada, Purusha, Vacana, dhatus};
use panini_prakriya::derive;

/// (root_id, lakara_label, [P.E, P.D, P.B, M.E, M.D, M.B, U.E, U.D, U.B]) in SLP1.
/// The first column is a `Dhatu::id`, not a `code` — see
/// `every_form_validates_and_matches`'s comment for why that distinction
/// matters (it is gaṇa-qualified where two roots share an SLP1 form, e.g.
/// svādi's `aS.5` vs kryādi's `aS`).
const PARADIGM: &[(&str, &str, [&str; 9])] = &[
    (
        "BU",
        "laT",
        [
            "Bavati", "BavataH", "Bavanti", "Bavasi", "BavaTaH", "BavaTa", "BavAmi", "BavAvaH",
            "BavAmaH",
        ],
    ),
    (
        "nI",
        "laT",
        [
            "nayati", "nayataH", "nayanti", "nayasi", "nayaTaH", "nayaTa", "nayAmi", "nayAvaH",
            "nayAmaH",
        ],
    ),
    (
        "ji",
        "laT",
        [
            "jayati", "jayataH", "jayanti", "jayasi", "jayaTaH", "jayaTa", "jayAmi", "jayAvaH",
            "jayAmaH",
        ],
    ),
    (
        "smf",
        "laT",
        [
            "smarati", "smarataH", "smaranti", "smarasi", "smaraTaH", "smaraTa", "smarAmi",
            "smarAvaH", "smarAmaH",
        ],
    ),
    (
        "paW",
        "laT",
        [
            "paWati", "paWataH", "paWanti", "paWasi", "paWaTaH", "paWaTa", "paWAmi", "paWAvaH",
            "paWAmaH",
        ],
    ),
    (
        "vad",
        "laT",
        [
            "vadati", "vadataH", "vadanti", "vadasi", "vadaTaH", "vadaTa", "vadAmi", "vadAvaH",
            "vadAmaH",
        ],
    ),
    (
        "BU",
        "laN",
        [
            "aBavat", "aBavatAm", "aBavan", "aBavaH", "aBavatam", "aBavata", "aBavam", "aBavAva",
            "aBavAma",
        ],
    ),
    (
        "nI",
        "laN",
        [
            "anayat", "anayatAm", "anayan", "anayaH", "anayatam", "anayata", "anayam", "anayAva",
            "anayAma",
        ],
    ),
    (
        "ji",
        "laN",
        [
            "ajayat", "ajayatAm", "ajayan", "ajayaH", "ajayatam", "ajayata", "ajayam", "ajayAva",
            "ajayAma",
        ],
    ),
    (
        "smf",
        "laN",
        [
            "asmarat",
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
        "paW",
        "laN",
        [
            "apaWat", "apaWatAm", "apaWan", "apaWaH", "apaWatam", "apaWata", "apaWam", "apaWAva",
            "apaWAma",
        ],
    ),
    (
        "vad",
        "laN",
        [
            "avadat", "avadatAm", "avadan", "avadaH", "avadatam", "avadata", "avadam", "avadAva",
            "avadAma",
        ],
    ),
    (
        "BU",
        "loT",
        [
            "Bavatu", "BavatAm", "Bavantu", "Bava", "Bavatam", "Bavata", "BavAni", "BavAva",
            "BavAma",
        ],
    ),
    (
        "nI",
        "loT",
        [
            "nayatu", "nayatAm", "nayantu", "naya", "nayatam", "nayata", "nayAni", "nayAva",
            "nayAma",
        ],
    ),
    (
        "ji",
        "loT",
        [
            "jayatu", "jayatAm", "jayantu", "jaya", "jayatam", "jayata", "jayAni", "jayAva",
            "jayAma",
        ],
    ),
    (
        "smf",
        "loT",
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
        "paW",
        "loT",
        [
            "paWatu", "paWatAm", "paWantu", "paWa", "paWatam", "paWata", "paWAni", "paWAva",
            "paWAma",
        ],
    ),
    (
        "vad",
        "loT",
        [
            "vadatu", "vadatAm", "vadantu", "vada", "vadatam", "vadata", "vadAni", "vadAva",
            "vadAma",
        ],
    ),
    (
        "BU",
        "viDiliN",
        [
            "Bavet", "BavetAm", "BaveyuH", "BaveH", "Bavetam", "Baveta", "Baveyam", "Baveva",
            "Bavema",
        ],
    ),
    (
        "nI",
        "viDiliN",
        [
            "nayet", "nayetAm", "nayeyuH", "nayeH", "nayetam", "nayeta", "nayeyam", "nayeva",
            "nayema",
        ],
    ),
    (
        "ji",
        "viDiliN",
        [
            "jayet", "jayetAm", "jayeyuH", "jayeH", "jayetam", "jayeta", "jayeyam", "jayeva",
            "jayema",
        ],
    ),
    (
        "smf",
        "viDiliN",
        [
            "smaret", "smaretAm", "smareyuH", "smareH", "smaretam", "smareta", "smareyam",
            "smareva", "smarema",
        ],
    ),
    (
        "paW",
        "viDiliN",
        [
            "paWet", "paWetAm", "paWeyuH", "paWeH", "paWetam", "paWeta", "paWeyam", "paWeva",
            "paWema",
        ],
    ),
    (
        "vad",
        "viDiliN",
        [
            "vadet", "vadetAm", "vadeyuH", "vadeH", "vadetam", "vadeta", "vadeyam", "vadeva",
            "vadema",
        ],
    ),
    (
        "eD",
        "laT",
        [
            "eDate", "eDete", "eDante", "eDase", "eDeTe", "eDaDve", "eDe", "eDAvahe", "eDAmahe",
        ],
    ),
    (
        "laB",
        "laT",
        [
            "laBate", "laBete", "laBante", "laBase", "laBeTe", "laBaDve", "laBe", "laBAvahe",
            "laBAmahe",
        ],
    ),
    (
        "sev",
        "laT",
        [
            "sevate", "sevete", "sevante", "sevase", "seveTe", "sevaDve", "seve", "sevAvahe",
            "sevAmahe",
        ],
    ),
    (
        "vft",
        "laT",
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
        "BAz",
        "laT",
        [
            "BAzate", "BAzete", "BAzante", "BAzase", "BAzeTe", "BAzaDve", "BAze", "BAzAvahe",
            "BAzAmahe",
        ],
    ),
    (
        "Ikz",
        "laT",
        [
            "Ikzate", "Ikzete", "Ikzante", "Ikzase", "IkzeTe", "IkzaDve", "Ikze", "IkzAvahe",
            "IkzAmahe",
        ],
    ),
    (
        "eD",
        "loT",
        [
            "eDatAm", "eDetAm", "eDantAm", "eDasva", "eDeTAm", "eDaDvam", "eDE", "eDAvahE",
            "eDAmahE",
        ],
    ),
    (
        "laB",
        "loT",
        [
            "laBatAm", "laBetAm", "laBantAm", "laBasva", "laBeTAm", "laBaDvam", "laBE", "laBAvahE",
            "laBAmahE",
        ],
    ),
    (
        "sev",
        "loT",
        [
            "sevatAm", "sevetAm", "sevantAm", "sevasva", "seveTAm", "sevaDvam", "sevE", "sevAvahE",
            "sevAmahE",
        ],
    ),
    (
        "vft",
        "loT",
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
        "BAz",
        "loT",
        [
            "BAzatAm", "BAzetAm", "BAzantAm", "BAzasva", "BAzeTAm", "BAzaDvam", "BAzE", "BAzAvahE",
            "BAzAmahE",
        ],
    ),
    (
        "Ikz",
        "loT",
        [
            "IkzatAm", "IkzetAm", "IkzantAm", "Ikzasva", "IkzeTAm", "IkzaDvam", "IkzE", "IkzAvahE",
            "IkzAmahE",
        ],
    ),
    (
        "eD",
        "laN",
        [
            "EData", "EDetAm", "EDanta", "EDaTAH", "EDeTAm", "EDaDvam", "EDe", "EDAvahi", "EDAmahi",
        ],
    ),
    (
        "laB",
        "laN",
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
        "sev",
        "laN",
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
        "vft",
        "laN",
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
        "BAz",
        "laN",
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
        "Ikz",
        "laN",
        [
            "Ekzata", "EkzetAm", "Ekzanta", "EkzaTAH", "EkzeTAm", "EkzaDvam", "Ekze", "EkzAvahi",
            "EkzAmahi",
        ],
    ),
    (
        "eD",
        "viDiliN",
        [
            "eDeta", "eDeyAtAm", "eDeran", "eDeTAH", "eDeyATAm", "eDeDvam", "eDeya", "eDevahi",
            "eDemahi",
        ],
    ),
    (
        "laB",
        "viDiliN",
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
        "sev",
        "viDiliN",
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
        "vft",
        "viDiliN",
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
        "BAz",
        "viDiliN",
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
        "Ikz",
        "viDiliN",
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
        "div",
        "laT",
        [
            "dIvyati", "dIvyataH", "dIvyanti", "dIvyasi", "dIvyaTaH", "dIvyaTa", "dIvyAmi",
            "dIvyAvaH", "dIvyAmaH",
        ],
    ),
    (
        "naS",
        "laT",
        [
            "naSyati", "naSyataH", "naSyanti", "naSyasi", "naSyaTaH", "naSyaTa", "naSyAmi",
            "naSyAvaH", "naSyAmaH",
        ],
    ),
    (
        "kup",
        "laT",
        [
            "kupyati", "kupyataH", "kupyanti", "kupyasi", "kupyaTaH", "kupyaTa", "kupyAmi",
            "kupyAvaH", "kupyAmaH",
        ],
    ),
    (
        "man",
        "laT",
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
        "yuD",
        "laT",
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
        "vid",
        "laT",
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
        "tud",
        "laT",
        [
            "tudati", "tudataH", "tudanti", "tudasi", "tudaTaH", "tudaTa", "tudAmi", "tudAvaH",
            "tudAmaH",
        ],
    ),
    (
        "liK",
        "laT",
        [
            "liKati", "liKataH", "liKanti", "liKasi", "liKaTaH", "liKaTa", "liKAmi", "liKAvaH",
            "liKAmaH",
        ],
    ),
    (
        "viS",
        "laT",
        [
            "viSati", "viSataH", "viSanti", "viSasi", "viSaTaH", "viSaTa", "viSAmi", "viSAvaH",
            "viSAmaH",
        ],
    ),
    (
        "juz",
        "laT",
        [
            "juzate", "juzete", "juzante", "juzase", "juzeTe", "juzaDve", "juze", "juzAvahe",
            "juzAmahe",
        ],
    ),
    (
        "vij",
        "laT",
        [
            "vijate", "vijete", "vijante", "vijase", "vijeTe", "vijaDve", "vije", "vijAvahe",
            "vijAmahe",
        ],
    ),
    (
        "gur",
        "laT",
        [
            "gurate", "gurete", "gurante", "gurase", "gureTe", "guraDve", "gure", "gurAvahe",
            "gurAmahe",
        ],
    ),
    (
        "div",
        "laN",
        [
            "adIvyat",
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
        "naS",
        "laN",
        [
            "anaSyat",
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
        "kup",
        "laN",
        [
            "akupyat",
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
        "man",
        "laN",
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
        "yuD",
        "laN",
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
        "vid",
        "laN",
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
        "tud",
        "laN",
        [
            "atudat", "atudatAm", "atudan", "atudaH", "atudatam", "atudata", "atudam", "atudAva",
            "atudAma",
        ],
    ),
    (
        "liK",
        "laN",
        [
            "aliKat", "aliKatAm", "aliKan", "aliKaH", "aliKatam", "aliKata", "aliKam", "aliKAva",
            "aliKAma",
        ],
    ),
    (
        "viS",
        "laN",
        [
            "aviSat", "aviSatAm", "aviSan", "aviSaH", "aviSatam", "aviSata", "aviSam", "aviSAva",
            "aviSAma",
        ],
    ),
    (
        "juz",
        "laN",
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
        "vij",
        "laN",
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
        "gur",
        "laN",
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
        "div",
        "loT",
        [
            "dIvyatu", "dIvyatAm", "dIvyantu", "dIvya", "dIvyatam", "dIvyata", "dIvyAni",
            "dIvyAva", "dIvyAma",
        ],
    ),
    (
        "naS",
        "loT",
        [
            "naSyatu", "naSyatAm", "naSyantu", "naSya", "naSyatam", "naSyata", "naSyAni",
            "naSyAva", "naSyAma",
        ],
    ),
    (
        "kup",
        "loT",
        [
            "kupyatu", "kupyatAm", "kupyantu", "kupya", "kupyatam", "kupyata", "kupyAni",
            "kupyAva", "kupyAma",
        ],
    ),
    (
        "man",
        "loT",
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
        "yuD",
        "loT",
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
        "vid",
        "loT",
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
        "tud",
        "loT",
        [
            "tudatu", "tudatAm", "tudantu", "tuda", "tudatam", "tudata", "tudAni", "tudAva",
            "tudAma",
        ],
    ),
    (
        "liK",
        "loT",
        [
            "liKatu", "liKatAm", "liKantu", "liKa", "liKatam", "liKata", "liKAni", "liKAva",
            "liKAma",
        ],
    ),
    (
        "viS",
        "loT",
        [
            "viSatu", "viSatAm", "viSantu", "viSa", "viSatam", "viSata", "viSAni", "viSAva",
            "viSAma",
        ],
    ),
    (
        "juz",
        "loT",
        [
            "juzatAm", "juzetAm", "juzantAm", "juzasva", "juzeTAm", "juzaDvam", "juzE", "juzAvahE",
            "juzAmahE",
        ],
    ),
    (
        "vij",
        "loT",
        [
            "vijatAm", "vijetAm", "vijantAm", "vijasva", "vijeTAm", "vijaDvam", "vijE", "vijAvahE",
            "vijAmahE",
        ],
    ),
    (
        "gur",
        "loT",
        [
            "guratAm", "guretAm", "gurantAm", "gurasva", "gureTAm", "guraDvam", "gurE", "gurAvahE",
            "gurAmahE",
        ],
    ),
    (
        "div",
        "viDiliN",
        [
            "dIvyet", "dIvyetAm", "dIvyeyuH", "dIvyeH", "dIvyetam", "dIvyeta", "dIvyeyam",
            "dIvyeva", "dIvyema",
        ],
    ),
    (
        "naS",
        "viDiliN",
        [
            "naSyet", "naSyetAm", "naSyeyuH", "naSyeH", "naSyetam", "naSyeta", "naSyeyam",
            "naSyeva", "naSyema",
        ],
    ),
    (
        "kup",
        "viDiliN",
        [
            "kupyet", "kupyetAm", "kupyeyuH", "kupyeH", "kupyetam", "kupyeta", "kupyeyam",
            "kupyeva", "kupyema",
        ],
    ),
    (
        "man",
        "viDiliN",
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
        "yuD",
        "viDiliN",
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
        "vid",
        "viDiliN",
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
        "tud",
        "viDiliN",
        [
            "tudet", "tudetAm", "tudeyuH", "tudeH", "tudetam", "tudeta", "tudeyam", "tudeva",
            "tudema",
        ],
    ),
    (
        "liK",
        "viDiliN",
        [
            "liKet", "liKetAm", "liKeyuH", "liKeH", "liKetam", "liKeta", "liKeyam", "liKeva",
            "liKema",
        ],
    ),
    (
        "viS",
        "viDiliN",
        [
            "viSet", "viSetAm", "viSeyuH", "viSeH", "viSetam", "viSeta", "viSeyam", "viSeva",
            "viSema",
        ],
    ),
    (
        "juz",
        "viDiliN",
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
        "vij",
        "viDiliN",
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
        "gur",
        "viDiliN",
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
        "yA",
        "laT",
        [
            "yAti", "yAtaH", "yAnti", "yAsi", "yATaH", "yATa", "yAmi", "yAvaH", "yAmaH",
        ],
    ),
    (
        "vA",
        "laT",
        [
            "vAti", "vAtaH", "vAnti", "vAsi", "vATaH", "vATa", "vAmi", "vAvaH", "vAmaH",
        ],
    ),
    (
        "yA",
        "laN",
        [
            "ayAt", "ayAtAm", "ayAn", "ayAH", "ayAtam", "ayAta", "ayAm", "ayAva", "ayAma",
        ],
    ),
    (
        "vA",
        "laN",
        [
            "avAt", "avAtAm", "avAn", "avAH", "avAtam", "avAta", "avAm", "avAva", "avAma",
        ],
    ),
    (
        "yA",
        "loT",
        [
            "yAtu", "yAtAm", "yAntu", "yAhi", "yAtam", "yAta", "yAni", "yAva", "yAma",
        ],
    ),
    (
        "vA",
        "loT",
        [
            "vAtu", "vAtAm", "vAntu", "vAhi", "vAtam", "vAta", "vAni", "vAva", "vAma",
        ],
    ),
    (
        "yA",
        "viDiliN",
        [
            "yAyAt", "yAyAtAm", "yAyuH", "yAyAH", "yAyAtam", "yAyAta", "yAyAm", "yAyAva", "yAyAma",
        ],
    ),
    (
        "vA",
        "viDiliN",
        [
            "vAyAt", "vAyAtAm", "vAyuH", "vAyAH", "vAyAtam", "vAyAta", "vAyAm", "vAyAva", "vAyAma",
        ],
    ),
    (
        "ad",
        "laT",
        [
            "atti", "attaH", "adanti", "atsi", "atTaH", "atTa", "admi", "advaH", "admaH",
        ],
    ),
    (
        "ad",
        "laN",
        [
            "Adat", "AttAm", "Adan", "AdaH", "Attam", "Atta", "Adam", "Adva", "Adma",
        ],
    ),
    (
        "ad",
        "loT",
        [
            "attu", "attAm", "adantu", "adDi", "attam", "atta", "adAni", "adAva", "adAma",
        ],
    ),
    (
        "ad",
        "viDiliN",
        [
            "adyAt", "adyAtAm", "adyuH", "adyAH", "adyAtam", "adyAta", "adyAm", "adyAva", "adyAma",
        ],
    ),
    (
        "As",
        "laT",
        [
            "Aste", "AsAte", "Asate", "Asse", "AsATe", "ADve", "Ase", "Asvahe", "Asmahe",
        ],
    ),
    (
        "As",
        "laN",
        [
            "Asta", "AsAtAm", "Asata", "AsTAH", "AsATAm", "ADvam", "Asi", "Asvahi", "Asmahi",
        ],
    ),
    (
        "As",
        "loT",
        [
            "AstAm", "AsAtAm", "AsatAm", "Assva", "AsATAm", "ADvam", "AsE", "AsAvahE", "AsAmahE",
        ],
    ),
    (
        "As",
        "viDiliN",
        [
            "AsIta", "AsIyAtAm", "AsIran", "AsITAH", "AsIyATAm", "AsIDvam", "AsIya", "AsIvahi",
            "AsImahi",
        ],
    ),
    (
        "vas",
        "laT",
        [
            "vaste", "vasAte", "vasate", "vasse", "vasATe", "vaDve", "vase", "vasvahe", "vasmahe",
        ],
    ),
    (
        "vas",
        "laN",
        [
            "avasta", "avasAtAm", "avasata", "avasTAH", "avasATAm", "avaDvam", "avasi", "avasvahi",
            "avasmahi",
        ],
    ),
    (
        "vas",
        "loT",
        [
            "vastAm", "vasAtAm", "vasatAm", "vassva", "vasATAm", "vaDvam", "vasE", "vasAvahE",
            "vasAmahE",
        ],
    ),
    (
        "vas",
        "viDiliN",
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
        "SI",
        "laT",
        [
            "Sete", "SayAte", "Serate", "Seze", "SayATe", "SeDve", "Saye", "Sevahe", "Semahe",
        ],
    ),
    (
        "SI",
        "laN",
        [
            "aSeta", "aSayAtAm", "aSerata", "aSeTAH", "aSayATAm", "aSeDvam", "aSayi", "aSevahi",
            "aSemahi",
        ],
    ),
    (
        "SI",
        "loT",
        [
            "SetAm", "SayAtAm", "SeratAm", "Sezva", "SayATAm", "SeDvam", "SayE", "SayAvahE",
            "SayAmahE",
        ],
    ),
    (
        "SI",
        "viDiliN",
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
        "kliS",
        "laT",
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
        "kliS",
        "laN",
        [
            "akliSnAt",
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
        "kliS",
        "loT",
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
        "kliS",
        "viDiliN",
        [
            "kliSnIyAt",
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
        "guD",
        "laT",
        [
            "guDnAti", "guDnItaH", "guDnanti", "guDnAsi", "guDnITaH", "guDnITa", "guDnAmi",
            "guDnIvaH", "guDnImaH",
        ],
    ),
    (
        "guD",
        "laN",
        [
            "aguDnAt",
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
        "guD",
        "loT",
        [
            "guDnAtu", "guDnItAm", "guDnantu", "guDAna", "guDnItam", "guDnIta", "guDnAni",
            "guDnAva", "guDnAma",
        ],
    ),
    (
        "guD",
        "viDiliN",
        [
            "guDnIyAt",
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
        "aS",
        "laT",
        [
            "aSnAti", "aSnItaH", "aSnanti", "aSnAsi", "aSnITaH", "aSnITa", "aSnAmi", "aSnIvaH",
            "aSnImaH",
        ],
    ),
    (
        "aS",
        "laN",
        [
            "ASnAt", "ASnItAm", "ASnan", "ASnAH", "ASnItam", "ASnIta", "ASnAm", "ASnIva", "ASnIma",
        ],
    ),
    (
        "aS",
        "loT",
        [
            "aSnAtu", "aSnItAm", "aSnantu", "aSAna", "aSnItam", "aSnIta", "aSnAni", "aSnAva",
            "aSnAma",
        ],
    ),
    (
        "aS",
        "viDiliN",
        [
            "aSnIyAt",
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
        "muz",
        "laT",
        [
            "muzRAti", "muzRItaH", "muzRanti", "muzRAsi", "muzRITaH", "muzRITa", "muzRAmi",
            "muzRIvaH", "muzRImaH",
        ],
    ),
    (
        "muz",
        "laN",
        [
            "amuzRAt",
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
        "muz",
        "loT",
        [
            "muzRAtu", "muzRItAm", "muzRantu", "muzARa", "muzRItam", "muzRIta", "muzRAni",
            "muzRAva", "muzRAma",
        ],
    ),
    (
        "muz",
        "viDiliN",
        [
            "muzRIyAt",
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
        "vrI",
        "laT",
        [
            "vrIRAti", "vrIRItaH", "vrIRanti", "vrIRAsi", "vrIRITaH", "vrIRITa", "vrIRAmi",
            "vrIRIvaH", "vrIRImaH",
        ],
    ),
    (
        "vrI",
        "laN",
        [
            "avrIRAt",
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
        "vrI",
        "loT",
        [
            "vrIRAtu", "vrIRItAm", "vrIRantu", "vrIRIhi", "vrIRItam", "vrIRIta", "vrIRAni",
            "vrIRAva", "vrIRAma",
        ],
    ),
    (
        "vrI",
        "viDiliN",
        [
            "vrIRIyAt",
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
        "vf",
        "laT",
        [
            "vfRIte", "vfRAte", "vfRate", "vfRIze", "vfRATe", "vfRIDve", "vfRe", "vfRIvahe",
            "vfRImahe",
        ],
    ),
    (
        "vf",
        "laN",
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
        "vf",
        "loT",
        [
            "vfRItAm", "vfRAtAm", "vfRatAm", "vfRIzva", "vfRATAm", "vfRIDvam", "vfRE", "vfRAvahE",
            "vfRAmahE",
        ],
    ),
    (
        "vf",
        "viDiliN",
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
        "Ap",
        "laT",
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
        "Ap",
        "laN",
        [
            "Apnot", "ApnutAm", "Apnuvan", "ApnoH", "Apnutam", "Apnuta", "Apnavam", "Apnuva",
            "Apnuma",
        ],
    ),
    (
        "Ap",
        "loT",
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
        "Ap",
        "viDiliN",
        [
            "ApnuyAt",
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
        "Sak",
        "laT",
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
        "Sak",
        "laN",
        [
            "aSaknot",
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
        "Sak",
        "loT",
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
        "Sak",
        "viDiliN",
        [
            "SaknuyAt",
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
        "hi",
        "laT",
        [
            "hinoti", "hinutaH", "hinvanti", "hinozi", "hinuTaH", "hinuTa", "hinomi", "hinuvaH",
            "hinumaH",
        ],
    ),
    (
        "hi",
        "laN",
        [
            "ahinot", "ahinutAm", "ahinvan", "ahinoH", "ahinutam", "ahinuta", "ahinavam",
            "ahinuva", "ahinuma",
        ],
    ),
    (
        "hi",
        "loT",
        [
            "hinotu", "hinutAm", "hinvantu", "hinu", "hinutam", "hinuta", "hinavAni", "hinavAva",
            "hinavAma",
        ],
    ),
    (
        "hi",
        "viDiliN",
        [
            "hinuyAt",
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
        "ri",
        "laT",
        [
            "riRoti", "riRutaH", "riRvanti", "riRozi", "riRuTaH", "riRuTa", "riRomi", "riRuvaH",
            "riRumaH",
        ],
    ),
    (
        "ri",
        "laN",
        [
            "ariRot", "ariRutAm", "ariRvan", "ariRoH", "ariRutam", "ariRuta", "ariRavam",
            "ariRuva", "ariRuma",
        ],
    ),
    (
        "ri",
        "loT",
        [
            "riRotu", "riRutAm", "riRvantu", "riRu", "riRutam", "riRuta", "riRavAni", "riRavAva",
            "riRavAma",
        ],
    ),
    (
        "ri",
        "viDiliN",
        [
            "riRuyAt",
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
        "aS.5",
        "laT",
        [
            "aSnute", "aSnuvAte", "aSnuvate", "aSnuze", "aSnuvATe", "aSnuDve", "aSnuve",
            "aSnuvahe", "aSnumahe",
        ],
    ),
    (
        "aS.5",
        "laN",
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
        "aS.5",
        "loT",
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
        "aS.5",
        "viDiliN",
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
        "stiG",
        "laT",
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
        "stiG",
        "laN",
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
        "stiG",
        "loT",
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
        "stiG",
        "viDiliN",
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
];

fn lan_a_form(id: &str, pu: Purusha, va: Vacana) -> String {
    let d = dhatus().iter().find(|d| d.id == id).unwrap();
    derive(d, Lakara::Lan, Pada::Atmanepada, pu, va).text()
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
        assert_eq!(lan_a_form("laB", pu, va), form, "{pu:?} {va:?}");
    }
}

#[test]
fn vowel_initial_roots_take_at_not_a() {
    // 6.4.72 āḍ ajādīnām (apavāda to 6.4.71) + 6.1.90 vṛddhi:
    // a+eD → ED (aidhata), a+Ikz → Ekz (aikṣata).
    assert_eq!(lan_a_form("eD", Purusha::Prathama, Vacana::Eka), "EData");
    assert_eq!(lan_a_form("Ikz", Purusha::Prathama, Vacana::Eka), "Ekzata");
}

#[test]
fn every_form_validates_and_matches() {
    let engine = Panini::new();
    for (root, lakara, forms) in PARADIGM {
        // `PARADIGM`'s first column is a `Dhatu::id` (gaṇa-qualified, so the
        // two √aś rows stay distinct: `aS.5` vs `aS`), but `Analysis::dhatu`
        // reports the surface `code` (deliberately not unique — it's a
        // user-facing spelling, not a key). The two must be resolved against
        // each other rather than compared directly. Because both √aś rows
        // share `code == "aS"`, matching on `code` alone would let a
        // mis-transcribed row silently bind to the WRONG root's forms as
        // long as the two roots' surfaces happen to be disjoint; `pada`
        // differs between the two (kryādi's is parasmaipada, svādi's is
        // ātmanepada), so pinning it too closes that hole.
        let d = dhatus().iter().find(|d| d.id == *root).unwrap();
        for expected in forms {
            let r = engine.check(expected);
            assert!(
                matches!(r.verdict, Verdict::Valid),
                "expected VALID for {expected} ({root} {lakara})"
            );
            assert!(
                r.analyses.iter().any(|a| a.form_slp1 == *expected
                    && a.dhatu == d.code
                    && a.pada == d.pada
                    && panini::lakara_name(a.lakara) == *lakara),
                "no {lakara} analysis of {root} produced {expected}"
            );
        }
    }
}

/// `every_form_validates_and_matches` only walks `PARADIGM`, so a root or
/// lakāra added to the enumerable space without golden rows would be checked
/// by nothing at all. This test closes that hole from the other side: every
/// (root × lakāra) pair the analyzer enumerates must either be pinned by a
/// `PARADIGM` block or appear in the explicit gated list below.
#[test]
fn paradigm_covers_every_enumerable_cell() {
    // adādi × vidhiliṅ was gated in slice 5a and ungated in slice 5b; √śī was
    // gated in slice 5f task 1 and ungated here. There are no gated cells any
    // more. This constant stays (empty) so the two assertions below keep
    // documenting that EVERY enumerable (root, lakara) pair must be pinned in
    // PARADIGM — a future partial slice may repopulate it, but it must never
    // silently hide a missing golden block.
    const GATED: &[(&str, &str)] = &[];

    let pinned: Vec<(&str, &str)> = PARADIGM.iter().map(|(r, l, _)| (*r, *l)).collect();
    let mut unpinned: Vec<(&str, &str)> = Vec::new();
    for d in dhatus() {
        for &lakara in panini_analyze::LAKARAS {
            let pair = (d.id, panini::lakara_name(lakara));
            if !pinned.contains(&pair) {
                unpinned.push(pair);
            }
        }
    }
    unpinned.sort_unstable();
    let mut gated = GATED.to_vec();
    gated.sort_unstable();
    assert_eq!(
        unpinned, gated,
        "every enumerable (root, lakara) pair needs golden rows in PARADIGM \
         (or an explicit entry in GATED, for a cell deliberately withheld from golden coverage)"
    );
    // Catches a duplicated PARADIGM block masking a missing one above.
    assert_eq!(
        PARADIGM.len() + GATED.len(),
        dhatus().len() * panini_analyze::LAKARAS.len(),
        "PARADIGM has a duplicate or stale (root, lakara) block"
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
        // Wrong pada: the root's pada tag gates the whole derivation
        // (1.3.12 / 1.3.78) and the analyzer proposes only the tagged pada.
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
        // Wrong pada: the root's pada tag gates the whole derivation.
        "manyati", // atmanepadin divādi root with a parasmaipada ending
        "vidyati", // atmanepadin divādi root, parasmaipada ending
        "tudate",  // parasmaipada tudādi root with an atmanepada ending
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
        "hinmaH", // the deliberate slice-5b gap the spec records: the
                  // elided-vikaraṇa alternate (hinvaH/hinmaH) is a second valid form
                  // not yet implemented, so it stays INVALID until 5b lands — see
                  // docs/superpowers/specs/2026-07-29-svadi-gana-design.md:117
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
