#![forbid(unsafe_code)]
use panini_analyze::candidates;
use panini_data::{Lakara, Pada, Purusha, Vacana};
use panini_lipi::{Scheme, from_slp1, normalize};
use panini_prakriya::{Prakriya, RuleStep, derive as derive_prakriya};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    Valid,
    Invalid,
}

pub struct Analysis {
    pub dhatu: String,
    pub lakara: Lakara,
    pub pada: Pada,
    pub purusha: Purusha,
    pub vacana: Vacana,
    pub form_slp1: String,
    pub trace: Vec<RuleStep>,
}

pub struct CheckResult {
    pub verdict: Verdict,
    pub input_slp1: String,
    pub detected: Scheme,
    pub analyses: Vec<Analysis>,
}

pub struct Panini;

impl Panini {
    pub fn new() -> Panini {
        Panini
    }

    pub fn check(&self, input: &str) -> CheckResult {
        let (slp1, detected) = normalize(input);
        let mut analyses = Vec::new();
        for c in candidates(&slp1) {
            let p = derive_prakriya(c.dhatu, c.lakara, c.pada, c.purusha, c.vacana);
            // A blocked prakriya derived nothing: its text is a partial
            // string (often the bare root code) that must never be reported
            // as a surface form — cf. the pada blocks in 1.3.12 / 1.3.78.
            if !p.blocked && p.text() == slp1 {
                analyses.push(Analysis {
                    dhatu: c.dhatu.code.to_string(),
                    lakara: c.lakara,
                    pada: c.pada,
                    purusha: c.purusha,
                    vacana: c.vacana,
                    form_slp1: p.text(),
                    trace: p.log,
                });
            }
        }
        let verdict = if analyses.is_empty() {
            Verdict::Invalid
        } else {
            Verdict::Valid
        };
        CheckResult {
            verdict,
            input_slp1: slp1,
            detected,
            analyses,
        }
    }

    pub fn derive(
        &self,
        dhatu: &panini_data::Dhatu,
        lakara: Lakara,
        pada: Pada,
        purusha: Purusha,
        vacana: Vacana,
    ) -> Prakriya {
        derive_prakriya(dhatu, lakara, pada, purusha, vacana)
    }
}

impl Default for Panini {
    fn default() -> Self {
        Panini::new()
    }
}

/// Render an SLP1 form in the requested scheme (for callers/CLI).
pub fn render(slp1: &str, scheme: Scheme) -> String {
    from_slp1(slp1, scheme)
}

/// SLP1 name of a lakāra, for display in traces and CLI output.
pub fn lakara_name(lakara: Lakara) -> &'static str {
    match lakara {
        Lakara::Lat => "laT",
        Lakara::Lan => "laN",
        Lakara::Lot => "loT",
        Lakara::VidhiLin => "viDiliN",
    }
}

/// SLP1 name of a pada, for display in traces and CLI output.
pub fn pada_name(pada: Pada) -> &'static str {
    match pada {
        Pada::Parasmaipada => "parasmEpadam",
        Pada::Atmanepada => "Atmanepadam",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_word_returns_trace() {
        let engine = Panini::new();
        let r = engine.check("bhavati");
        assert!(matches!(r.verdict, Verdict::Valid));
        let a = r.analyses.iter().find(|a| a.form_slp1 == "Bavati").unwrap();
        assert_eq!(a.dhatu, "BU");
        assert!(!a.trace.is_empty());
    }

    #[test]
    fn devanagari_input_is_accepted() {
        let engine = Panini::new();
        let r = engine.check("भवति");
        assert!(matches!(r.verdict, Verdict::Valid));
    }

    #[test]
    fn non_covered_word_is_invalid() {
        let engine = Panini::new();
        let r = engine.check("xyzq");
        assert!(matches!(r.verdict, Verdict::Invalid));
        assert!(r.analyses.is_empty());
    }

    #[test]
    fn analysis_reports_its_lakara() {
        let engine = Panini::new();
        let r = engine.check("Bavati");
        let a = r.analyses.iter().find(|a| a.form_slp1 == "Bavati").unwrap();
        assert!(matches!(a.lakara, Lakara::Lat));
        assert_eq!(lakara_name(a.lakara), "laT");
    }

    #[test]
    fn vidhilin_has_an_slp1_name() {
        assert_eq!(lakara_name(Lakara::VidhiLin), "viDiliN");
    }

    #[test]
    fn vidhilin_form_checks_valid() {
        let engine = Panini::new();
        let r = engine.check("Bavet");
        assert!(matches!(r.verdict, Verdict::Valid));
        let a = r.analyses.iter().find(|a| a.form_slp1 == "Bavet").unwrap();
        assert_eq!(a.dhatu, "BU");
        assert!(matches!(a.lakara, Lakara::VidhiLin));
    }

    #[test]
    fn analysis_reports_its_pada() {
        let engine = Panini::new();
        let r = engine.check("Bavati");
        let a = r.analyses.iter().find(|a| a.form_slp1 == "Bavati").unwrap();
        assert!(matches!(a.pada, Pada::Parasmaipada));
        assert_eq!(pada_name(a.pada), "parasmEpadam");
        assert_eq!(pada_name(Pada::Atmanepada), "Atmanepadam");
    }
}
