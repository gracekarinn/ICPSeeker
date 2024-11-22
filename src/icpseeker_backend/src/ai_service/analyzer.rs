use crate::models::cv::{CV, CVAnalysisStatus};
use crate::storage::memory::CVStorage;
use ic_cdk::api::time;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisFeedback {
    total_score: f32,
    section_scores: HashMap<String, SectionScore>,
    priority_improvements: Vec<String>,
    overall_feedback: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionScore {
    score: f32,
    feedback: String,
    suggestions: Vec<String>,
}

pub struct CVAnalyzer;

impl CVAnalyzer {
    pub async fn analyze_cv(cv_id: String) -> Result<(), String> {
        let mut cv = match CVStorage::get_cv(&cv_id) {
            Ok(cv) => cv,
            Err(e) => return Err(format!("Failed to retrieve CV: {}", e)),
        };

        cv.ai_analysis_status = CVAnalysisStatus::InProgress;
        if let Err(e) = CVStorage::update_cv(cv.clone()) {
            return Err(format!("Failed to update CV status: {}", e));
        }

        let analysis_result = Self::perform_analysis(&cv.content);

        cv.ai_analysis_status = CVAnalysisStatus::Completed;
        cv.ai_feedback = Some(serde_json::to_string(&analysis_result).map_err(|e| e.to_string())?);
        cv.updated_at = time();

        CVStorage::update_cv(cv).map_err(|e| format!("Failed to store analysis results: {}", e))?;

        Ok(())
    }

    fn perform_analysis(content: &str) -> AnalysisFeedback {
        let mut section_scores = HashMap::new();
        
        section_scores.insert(
            "contact".to_string(),
            Self::analyze_contact_section(content)
        );
        section_scores.insert(
            "experience".to_string(),
            Self::analyze_experience_section(content)
        );
        section_scores.insert(
            "education".to_string(),
            Self::analyze_education_section(content)
        );
        section_scores.insert(
            "skills".to_string(),
            Self::analyze_skills_section(content)
        );

        let total_score = Self::calculate_total_score(&section_scores);

        let priority_improvements = Self::generate_priority_improvements(&section_scores);
        let overall_feedback = Self::generate_overall_feedback(&section_scores, total_score);

        AnalysisFeedback {
            total_score,
            section_scores,
            priority_improvements,
            overall_feedback,
        }
    }

    fn analyze_contact_section(content: &str) -> SectionScore {
        let mut score = 0.0;
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
                "Contact information is complete".to_string()
            } else {
                "Contact information needs improvement".to_string()
            },
            suggestions,
        }
    }

    fn analyze_experience_section(content: &str) -> SectionScore {
        let mut score = 0.0;
        let mut suggestions = Vec::new();

        let keywords = vec!["experience", "work", "position", "role"];
        let keyword_count = keywords.iter()
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

        SectionScore {
            score: score.min(10.0),
            feedback: if score >= 7.0 {
                "Experience section is well detailed".to_string()
            } else {
                "Experience section needs more detail".to_string()
            },
            suggestions,
        }
    }

    fn analyze_education_section(content: &str) -> SectionScore {
        let mut score = 0.0;
        let mut suggestions = Vec::new();

        let keywords = vec!["education", "university", "degree", "school"];
        let keyword_count = keywords.iter()
            .filter(|&keyword| content.to_lowercase().contains(keyword))
            .count();

        score += (keyword_count as f32) * 2.5;

        if score < 5.0 {
            suggestions.push("Add more details about your education".to_string());
        }

        SectionScore {
            score: score.min(10.0),
            feedback: if score >= 7.0 {
                "Education section is complete".to_string()
            } else {
                "Education section needs more detail".to_string()
            },
            suggestions,
        }
    }

    fn analyze_skills_section(content: &str) -> SectionScore {
        let mut score = 0.0;
        let mut suggestions = Vec::new();

        let keywords = vec!["skills", "proficient", "expertise", "competencies"];
        let keyword_count = keywords.iter()
            .filter(|&keyword| content.to_lowercase().contains(keyword))
            .count();

        score += (keyword_count as f32) * 2.5;

        if score < 5.0 {
            suggestions.push("List your skills more clearly".to_string());
        }

        SectionScore {
            score: score.min(10.0),
            feedback: if score >= 7.0 {
                "Skills are well presented".to_string()
            } else {
                "Skills section could be improved".to_string()
            },
            suggestions,
        }
    }

    fn calculate_total_score(section_scores: &HashMap<String, SectionScore>) -> f32 {
        let weights: HashMap<&str, f32> = [
            ("contact", 0.2),
            ("experience", 0.3),
            ("education", 0.25),
            ("skills", 0.25),
        ].iter().cloned().collect();

        let mut total_score = 0.0;
        for (section, score) in section_scores {
            if let Some(&weight) = weights.get(section.as_str()) {
                total_score += score.score * weight;
            }
        }

        total_score
    }

    fn generate_priority_improvements(section_scores: &HashMap<String, SectionScore>) -> Vec<String> {
        let mut improvements = Vec::new();
        
        for (_, score) in section_scores {
            if score.score < 7.0 {
                improvements.extend(score.suggestions.clone());
            }
        }

        improvements.truncate(5);
        improvements
    }

    fn generate_overall_feedback(section_scores: &HashMap<String, SectionScore>, total_score: f32) -> String {
        let base_feedback = if total_score >= 8.0 {
            "Your CV is well-structured and comprehensive. "
        } else if total_score >= 6.0 {
            "Your CV is good but has room for improvement. "
        } else {
            "Your CV needs significant improvements. "
        };

        let mut feedback = String::from(base_feedback);

        for (section, score) in section_scores {
            if score.score < 7.0 {
                feedback.push_str(&format!("{}. ", score.feedback));
            }
        }

        feedback
    }
}