use panini::{Panini, Verdict};
use panini_data::{Lakara, Pada, Purusha, Vacana, dhatus};
use panini_prakriya::derive;

/// (root_code, lakara_label, [P.E, P.D, P.B, M.E, M.D, M.B, U.E, U.D, U.B]) in SLP1.
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
            "smaratu", "smaratAm", "smarantu", "smara", "smaratam", "smarata", "smarAni",
            "smarAva", "smarAma",
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
];

fn lan_a_form(code: &str, pu: Purusha, va: Vacana) -> String {
    let d = dhatus().iter().find(|d| d.code == code).unwrap();
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
        for expected in forms {
            let r = engine.check(expected);
            assert!(
                matches!(r.verdict, Verdict::Valid),
                "expected VALID for {expected} ({root} {lakara})"
            );
            assert!(
                r.analyses.iter().any(|a| a.form_slp1 == *expected
                    && a.dhatu == *root
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
    // adādi × vidhiliṅ was gated in slice 5a and ungated in slice 5b; there
    // are no gated cells any more. This constant stays (empty) so the two
    // assertions below keep documenting that EVERY enumerable (root, lakara)
    // pair must be pinned in PARADIGM — a future partial slice may repopulate
    // it, but it must never silently hide a missing golden block.
    const GATED: &[(&str, &str)] = &[];

    let pinned: Vec<(&str, &str)> = PARADIGM.iter().map(|(r, l, _)| (*r, *l)).collect();
    let mut unpinned: Vec<(&str, &str)> = Vec::new();
    for d in dhatus() {
        for &lakara in panini_analyze::LAKARAS {
            let pair = (d.code, panini::lakara_name(lakara));
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
    ] {
        assert!(
            matches!(engine.check(bad).verdict, Verdict::Invalid),
            "expected INVALID for {bad}"
        );
    }
}
