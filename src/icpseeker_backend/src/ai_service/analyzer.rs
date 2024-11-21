use crate::models::cv::CV;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SectionScore {
    score: f32,
    feedback: String,
    suggestions: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct CVAnalysisResult {
    total_score: f32,
    section_scores: HashMap<String, SectionScore>,
    priority_improvements: Vec<String>,
    overall_feedback: String,
}

const WEIGHTS: HashMap<&str, f32> = HashMap::from([
    ("contact", 0.10),
    ("summary", 0.15),
    ("experience", 0.25),
    ("education", 0.20),
    ("skills", 0.20),
    ("format", 0.10),
]);

struct AnalysisConfig {
    min_section_length: usize,
    required_sections: Vec<&'static str>,
    skill_keywords: Vec<&'static str>,
    experience_keywords: Vec<&'static str>,
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            min_section_length: 50,
            required_sections: vec!["contact", "experience", "education", "skills"],
            skill_keywords: vec!["experience with", "proficient in", "skills", "expertise"],
            experience_keywords: vec!["work experience", "employment", "position", "role"],
        }
    }
}

pub fn analyze_cv(cv: &CV) -> CVAnalysisResult {
    let config = AnalysisConfig::default();
    let mut section_scores = HashMap::new();
    
    section_scores.insert(
        "contact".to_string(),
        analyze_contact_section(&cv.content)
    );
    section_scores.insert(
        "summary".to_string(),
        analyze_summary_section(&cv.content)
    );
    section_scores.insert(
        "experience".to_string(),
        analyze_experience_section(&cv.content, &config)
    );
    section_scores.insert(
        "education".to_string(),
        analyze_education_section(&cv.content)
    );
    section_scores.insert(
        "skills".to_string(),
        analyze_skills_section(&cv.content, &config)
    );
    section_scores.insert(
        "format".to_string(),
        analyze_format(&cv.content)
    );

    let total_score = calculate_total_score(&section_scores);

    let priority_improvements = generate_priority_improvements(&section_scores);

    let overall_feedback = generate_overall_feedback(&section_scores, total_score);

    CVAnalysisResult {
        total_score,
        section_scores,
        priority_improvements,
        overall_feedback,
    }
}

fn analyze_contact_section(content: &str) -> SectionScore {
    let mut score = 0.0;
    let mut feedback = Vec::new();
    let mut suggestions = Vec::new();

    if content.to_lowercase().contains('@') {
        score += 3.0;
    } else {
        suggestions.push("Add an email address".to_string());
    }

    if content.matches(char::is_numeric).count() >= 10 {
        score += 3.0;
    } else {
        suggestions.push("Add a phone number".to_string());
    }

    if content.to_lowercase().contains("address") || content.to_lowercase().contains("location") {
        score += 4.0;
    } else {
        suggestions.push("Add your location".to_string());
    }

    SectionScore {
        score,
        feedback: if score >= 8.0 {
            "Contact information is well-structured".to_string()
        } else {
            "Contact information needs improvement".to_string()
        },
        suggestions,
    }
}

fn analyze_experience_section(content: &str, config: &AnalysisConfig) -> SectionScore {
    let mut score = 0.0;
    let mut feedback = Vec::new();
    let mut suggestions = Vec::new();

    let keyword_count = config.experience_keywords.iter()
        .filter(|&keyword| content.to_lowercase().contains(keyword))
        .count();

    score += (keyword_count as f32) * 2.0;

    let has_dates = content.matches(char::is_numeric)
        .filter(|num| num.len() == 4)
        .count() >= 2;

    if has_dates {
        score += 3.0;
    } else {
        suggestions.push("Add dates to your work experience".to_string());
    }

    if content.contains('•') || content.contains('-') || content.contains(". ") {
        score += 2.0;
    } else {
        suggestions.push("Use bullet points to describe your experiences".to_string());
    }

    SectionScore {
        score: score.min(10.0),
        feedback: if score >= 7.0 {
            "Experience section is detailed".to_string()
        } else {
            "Experience section needs more detail".to_string()
        },
        suggestions,
    }
}

fn analyze_skills_section(content: &str, config: &AnalysisConfig) -> SectionScore {
    let mut score = 0.0;
    let mut suggestions = Vec::new();

    let has_skills_section = config.skill_keywords.iter()
        .any(|&keyword| content.to_lowercase().contains(keyword));

    if has_skills_section {
        score += 5.0;
    } else {
        suggestions.push("Add a dedicated skills section".to_string());
    }

    if content.contains(':') || content.contains('•') {
        score += 3.0;
    } else {
        suggestions.push("Categorize your skills".to_string());
    }

    let word_count = content.split_whitespace().count();
    if word_count >= 15 {
        score += 2.0;
    } else {
        suggestions.push("List more relevant skills".to_string());
    }

    SectionScore {
        score,
        feedback: if score >= 8.0 {
            "Skills section is well-organized".to_string()
        } else {
            "Skills section needs better organization".to_string()
        },
        suggestions,
    }
}

fn calculate_total_score(section_scores: &HashMap<String, SectionScore>) -> f32 {
    let mut total_score = 0.0;

    for (section, weight) in WEIGHTS {
        if let Some(score) = section_scores.get(section) {
            total_score += score.score * weight;
        }
    }

    total_score
}

fn generate_priority_improvements(section_scores: &HashMap<String, SectionScore>) -> Vec<String> {
    let mut improvements = Vec::new();
    
    for (section, score) in section_scores {
        if score.score < 7.0 {
            improvements.extend(score.suggestions.clone());
        }
    }

    improvements.truncate(5);
    improvements
}

fn generate_overall_feedback(section_scores: &HashMap<String, SectionScore>, total_score: f32) -> String {
    let mut feedback = String::new();

    if total_score >= 8.0 {
        feedback.push_str("Your CV is well-structured and comprehensive. ");
    } else if total_score >= 6.0 {
        feedback.push_str("Your CV is good but has room for improvement. ");
    } else {
        feedback.push_str("Your CV needs significant improvements. ");
    }

    for (section, score) in section_scores {
        if score.score < 7.0 {
            feedback.push_str(&format!("{}. ", score.feedback));
        }
    }

    feedback
}

fn analyze_summary_section(content: &str) -> SectionScore {
    SectionScore {
        score: 7.0,
        feedback: "Summary section is present".to_string(),
        suggestions: vec![],
    }
}

fn analyze_education_section(content: &str) -> SectionScore {
    SectionScore {
        score: 7.0,
        feedback: "Education section is present".to_string(),
        suggestions: vec![],
    }
}

fn analyze_format(content: &str) -> SectionScore {
    SectionScore {
        score: 7.0,
        feedback: "Format is acceptable".to_string(),
        suggestions: vec![],
    }
}