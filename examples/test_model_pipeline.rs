//! Test the complete specialized model pipeline
//! 
//! This example validates that all specialized models work correctly
//! and can be chained together for complete game development workflows.

use std::time::Instant;
use anyhow::Result;
use log::{info, warn, error};

#[cfg(feature = "training")]
use lumina_ai::{AiConfig, AiProvider, initialize_ai_system, GamePrompt};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    println!("🧪 Lumina Specialized Model Pipeline Test");
    println!("========================================");

    #[cfg(not(feature = "training"))]
    {
        println!("Training features not enabled. Run with:");
        println!("cargo run --example test_model_pipeline --features training");
        return Ok(());
    }

    #[cfg(feature = "training")]
    {
        // Test individual models
        println!("🔬 Testing Individual Models");
        println!("============================");

        let model_tests = vec![
            ("lumina-design", "Design a unique puzzle mechanic for a platformer game"),
            ("lumina-scene", "Create SDL for a simple space shooter with enemies and power-ups"),
            ("lumina-assets", "Generate sprite specifications for a medieval fantasy knight character"),
            ("lumina-scripts", "Create player movement and jumping logic for a 2D platformer"),
            ("lumina-perf", "Analyze performance bottlenecks in a particle-heavy explosion system"),
            ("lumina-deploy", "Configure build settings and store listing for Steam release"),
        ];

        let mut test_results = Vec::new();

        for (model_name, test_prompt) in model_tests {
            println!("\n🎯 Testing {}", model_name);
            println!("Prompt: {}", test_prompt);
            
            let test_start = Instant::now();
            
            match test_individual_model(model_name, test_prompt).await {
                Ok(response) => {
                    let test_time = test_start.elapsed();
                    println!("✅ Success ({:.2}s)", test_time.as_secs_f32());
                    println!("Response length: {} characters", response.len());
                    
                    // Validate response quality
                    let quality_score = validate_response_quality(model_name, &response);
                    println!("Quality score: {:.1}/10", quality_score);
                    
                    test_results.push((model_name, true, test_time.as_secs_f32(), quality_score));
                }
                Err(e) => {
                    let test_time = test_start.elapsed();
                    println!("❌ Failed ({:.2}s): {}", test_time.as_secs_f32(), e);
                    test_results.push((model_name, false, test_time.as_secs_f32(), 0.0));
                }
            }
        }

        // Test complete workflow
        println!("\n🔗 Testing Complete Workflow");
        println!("============================");
        
        match test_complete_workflow().await {
            Ok(_) => println!("✅ Complete workflow test passed"),
            Err(e) => println!("❌ Complete workflow test failed: {}", e),
        }

        // Display test summary
        println!("\n📊 Test Results Summary");
        println!("=======================");
        
        let passed_tests = test_results.iter().filter(|(_, success, _, _)| *success).count();
        let total_tests = test_results.len();
        let avg_response_time = test_results.iter()
            .map(|(_, _, time, _)| time)
            .sum::<f32>() / total_tests as f32;
        let avg_quality = test_results.iter()
            .filter(|(_, success, _, _)| *success)
            .map(|(_, _, _, quality)| quality)
            .sum::<f32>() / passed_tests as f32;

        println!("┌─────────────────┬─────────┬──────────┬───────────┐");
        println!("│ Model           │ Status  │ Time (s) │ Quality   │");
        println!("├─────────────────┼─────────┼──────────┼───────────┤");
        
        for (model, success, time, quality) in &test_results {
            let status = if *success { "✅ Pass" } else { "❌ Fail" };
            let quality_str = if *success { format!("{:.1}/10", quality) } else { "N/A".to_string() };
            println!("│ {:15} │ {:7} │ {:8.2} │ {:9} │", model, status, time, quality_str);
        }
        
        println!("└─────────────────┴─────────┴──────────┴───────────┘");
        
        println!("\n📈 Overall Statistics:");
        println!("  Tests passed: {}/{}", passed_tests, total_tests);
        println!("  Success rate: {:.1}%", (passed_tests as f32 / total_tests as f32) * 100.0);
        println!("  Average response time: {:.2}s", avg_response_time);
        if passed_tests > 0 {
            println!("  Average quality score: {:.1}/10", avg_quality);
        }

        // Performance benchmarks
        println!("\n⚡ Performance Benchmarks:");
        println!("  Target response time: <30s per model");
        println!("  Target quality score: >7.0/10");
        println!("  Target success rate: >90%");

        let meets_performance = avg_response_time < 30.0;
        let meets_quality = avg_quality > 7.0;
        let meets_success = (passed_tests as f32 / total_tests as f32) > 0.9;

        println!("\n🎯 Benchmark Results:");
        println!("  Response time: {} (target: <30s)", 
                if meets_performance { "✅ PASS" } else { "❌ FAIL" });
        println!("  Quality score: {} (target: >7.0)", 
                if meets_quality { "✅ PASS" } else { "❌ FAIL" });
        println!("  Success rate: {} (target: >90%)", 
                if meets_success { "✅ PASS" } else { "❌ FAIL" });

        if meets_performance && meets_quality && meets_success {
            println!("\n🏆 All benchmarks passed! Your specialized models are ready for production.");
        } else {
            println!("\n⚠️  Some benchmarks failed. Consider retraining or optimizing models.");
        }

        // Usage recommendations
        println!("\n💡 Usage Recommendations:");
        for (model, success, time, quality) in &test_results {
            if *success {
                if *time < 15.0 && *quality > 8.0 {
                    println!("  • {}: Excellent performance - ready for production", model);
                } else if *time < 30.0 && *quality > 6.0 {
                    println!("  • {}: Good performance - suitable for most use cases", model);
                } else {
                    println!("  • {}: Acceptable performance - consider optimization", model);
                }
            } else {
                println!("  • {}: Failed tests - requires attention before use", model);
            }
        }
    }

    Ok(())
}

#[cfg(feature = "training")]
async fn test_individual_model(model_name: &str, prompt: &str) -> Result<String> {
    let config = AiConfig {
        provider: AiProvider::Ollama,
        text_model: format!("{}:latest", model_name),
        base_url: Some("http://localhost:11434".to_string()),
        ..Default::default()
    };

    let mut generator = initialize_ai_system(config).await?;
    
    let game_prompt = GamePrompt::new(prompt);
    let request = lumina_ai::GameGenerationRequest::new(game_prompt);
    
    // For testing, we'll use a simplified approach
    // In practice, different models might need different request types
    match generator.generate_game(request).await {
        Ok(result) => Ok(result.scene.to_json()?),
        Err(e) => Err(e),
    }
}

#[cfg(feature = "training")]
async fn test_complete_workflow() -> Result<()> {
    println!("🔄 Testing multi-model workflow...");
    
    // Step 1: Design concept
    println!("  1. Generating game concept with lumina-design...");
    let design_response = test_individual_model(
        "lumina-design", 
        "Design a unique tower defense game with environmental mechanics"
    ).await?;
    
    // Step 2: Create scene
    println!("  2. Converting concept to scene with lumina-scene...");
    let scene_prompt = format!("Convert this game concept to SDL: {}", 
                              design_response.chars().take(500).collect::<String>());
    let scene_response = test_individual_model("lumina-scene", &scene_prompt).await?;
    
    // Step 3: Generate assets
    println!("  3. Creating asset specifications with lumina-assets...");
    let asset_response = test_individual_model(
        "lumina-assets", 
        "Create sprite specifications for tower defense game elements"
    ).await?;
    
    println!("✅ Complete workflow test successful");
    println!("   Generated {} chars of design content", design_response.len());
    println!("   Generated {} chars of scene content", scene_response.len());
    println!("   Generated {} chars of asset specs", asset_response.len());
    
    Ok(())
}

#[cfg(feature = "training")]
fn validate_response_quality(model_name: &str, response: &str) -> f32 {
    let mut score = 0.0;
    let mut max_score = 0.0;

    // Basic response validation
    max_score += 2.0;
    if !response.is_empty() && response.len() > 50 {
        score += 2.0;
    }

    // Model-specific validation
    match model_name {
        "lumina-design" => {
            max_score += 8.0;
            
            // Check for design concepts
            if response.to_lowercase().contains("mechanic") { score += 1.0; }
            if response.to_lowercase().contains("player") { score += 1.0; }
            if response.to_lowercase().contains("balance") { score += 1.0; }
            if response.to_lowercase().contains("objective") { score += 1.0; }
            if response.to_lowercase().contains("progression") { score += 1.0; }
            if response.to_lowercase().contains("challenge") { score += 1.0; }
            if response.len() > 200 { score += 1.0; }
            if response.lines().count() > 5 { score += 1.0; }
        }
        "lumina-scene" => {
            max_score += 8.0;
            
            // Check for SDL structure
            if response.contains("metadata") { score += 2.0; }
            if response.contains("entities") { score += 2.0; }
            if response.contains("components") { score += 1.0; }
            if response.contains("Transform") { score += 1.0; }
            if response.contains("scripts") { score += 1.0; }
            
            // Check if it's valid JSON
            if serde_json::from_str::<serde_json::Value>(response).is_ok() {
                score += 1.0;
            }
        }
        "lumina-assets" => {
            max_score += 8.0;
            
            // Check for asset specifications
            if response.to_lowercase().contains("sprite") { score += 1.0; }
            if response.to_lowercase().contains("texture") { score += 1.0; }
            if response.to_lowercase().contains("color") { score += 1.0; }
            if response.to_lowercase().contains("size") || response.to_lowercase().contains("dimensions") { score += 1.0; }
            if response.to_lowercase().contains("format") { score += 1.0; }
            if response.to_lowercase().contains("resolution") { score += 1.0; }
            if response.contains("x") && response.chars().any(|c| c.is_numeric()) { score += 1.0; }
            if response.len() > 300 { score += 1.0; }
        }
        "lumina-scripts" => {
            max_score += 8.0;
            
            // Check for scripting concepts
            if response.to_lowercase().contains("event") { score += 1.0; }
            if response.to_lowercase().contains("trigger") { score += 1.0; }
            if response.to_lowercase().contains("action") { score += 1.0; }
            if response.to_lowercase().contains("condition") { score += 1.0; }
            if response.to_lowercase().contains("state") { score += 1.0; }
            if response.to_lowercase().contains("input") { score += 1.0; }
            if response.to_lowercase().contains("collision") { score += 1.0; }
            if response.to_lowercase().contains("system") { score += 1.0; }
        }
        "lumina-perf" => {
            max_score += 8.0;
            
            // Check for performance concepts
            if response.to_lowercase().contains("optimization") { score += 1.0; }
            if response.to_lowercase().contains("performance") { score += 1.0; }
            if response.to_lowercase().contains("memory") { score += 1.0; }
            if response.to_lowercase().contains("cpu") || response.to_lowercase().contains("gpu") { score += 1.0; }
            if response.to_lowercase().contains("bottleneck") { score += 1.0; }
            if response.to_lowercase().contains("fps") || response.to_lowercase().contains("frame") { score += 1.0; }
            if response.to_lowercase().contains("cache") { score += 1.0; }
            if response.to_lowercase().contains("profile") { score += 1.0; }
        }
        "lumina-deploy" => {
            max_score += 8.0;
            
            // Check for deployment concepts
            if response.to_lowercase().contains("platform") { score += 1.0; }
            if response.to_lowercase().contains("build") { score += 1.0; }
            if response.to_lowercase().contains("release") { score += 1.0; }
            if response.to_lowercase().contains("store") { score += 1.0; }
            if response.to_lowercase().contains("steam") || response.to_lowercase().contains("mobile") { score += 1.0; }
            if response.to_lowercase().contains("marketing") { score += 1.0; }
            if response.to_lowercase().contains("config") { score += 1.0; }
            if response.to_lowercase().contains("publish") { score += 1.0; }
        }
        _ => {
            max_score += 8.0;
            score += 4.0; // Default score for unknown models
        }
    }

    (score / max_score) * 10.0
}

#[cfg(feature = "training")]
mod test_utils {
    use super::*;
    
    pub fn benchmark_response_time(time: f32) -> &'static str {
        match time {
            t if t < 10.0 => "Excellent",
            t if t < 30.0 => "Good", 
            t if t < 60.0 => "Acceptable",
            _ => "Slow"
        }
    }
    
    pub fn benchmark_quality_score(score: f32) -> &'static str {
        match score {
            s if s >= 9.0 => "Excellent",
            s if s >= 7.0 => "Good",
            s if s >= 5.0 => "Acceptable", 
            s if s >= 3.0 => "Poor",
            _ => "Failed"
        }
    }
    
    pub fn generate_performance_report(results: &[(String, bool, f32, f32)]) -> String {
        let mut report = String::new();
        report.push_str("# Performance Report\n\n");
        
        for (model, success, time, quality) in results {
            report.push_str(&format!(
                "## {}\n- Status: {}\n- Response Time: {:.2}s ({})\n- Quality: {:.1}/10 ({})\n\n",
                model,
                if *success { "✅ Pass" } else { "❌ Fail" },
                time,
                benchmark_response_time(*time),
                quality,
                benchmark_quality_score(*quality)
            ));
        }
        
        report
    }
}