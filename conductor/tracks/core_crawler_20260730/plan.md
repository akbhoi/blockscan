# Track Implementation Plan: Core Bot-Blocking Detection & In-Domain Link Spidering CLI Engine

## Phase 1: URL Normalization & Domain Confinement Engine
- [ ] Task: Write Unit Tests for URL Normalization and Domain Confinement Logic
- [ ] Task: Implement URL Normalizer & Strict TLD Filter Module (`src/url_utils.rs`)
- [ ] Task: Phase Verification & Checkpoint (Refer to workflow.md)

## Phase 2: Bot-Blocking Detection & HTTP Scanner Module
- [ ] Task: Write Unit Tests for Detection Signatures (403, 429, WAF/CAPTCHA HTML body)
- [ ] Task: Implement HTTP Client & Bot-Blocking Scanner (`src/scanner.rs`)
- [ ] Task: Phase Verification & Checkpoint (Refer to workflow.md)

## Phase 3: Recursive Spidering & Concurrency Engine
- [ ] Task: Write Unit Tests for Concurrent Visited Tracker & Crawler Engine
- [ ] Task: Implement Async Crawler Engine with Semaphore Concurrency & Depth Bounds (`src/crawler.rs`)
- [ ] Task: Phase Verification & Checkpoint (Refer to workflow.md)

## Phase 4: CLI Interface & Tabular Output Integration
- [ ] Task: Integrate Clap Arguments, Comfy-Table, and Indicatif Spinner (`src/main.rs`)
- [ ] Task: End-to-End Integration Testing & Phase Verification Checkpoint (Refer to workflow.md)
