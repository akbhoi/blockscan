# Track Implementation Plan: Core Bot-Blocking Detection & In-Domain Link Spidering CLI Engine

## Phase 1: URL Normalization & Domain Confinement Engine [checkpoint: a2abdce]
- [x] Task: Write Unit Tests for URL Normalization and Domain Confinement Logic (a2abdce)
- [x] Task: Implement URL Normalizer & Strict TLD Filter Module (`src/url_utils.rs`) (a2abdce)
- [x] Task: Phase Verification & Checkpoint (Refer to workflow.md) (a2abdce)

## Phase 2: Bot-Blocking Detection & HTTP Scanner Module [checkpoint: 35a3637]
- [x] Task: Write Unit Tests for Detection Signatures (403, 429, WAF/CAPTCHA HTML body) (35a3637)
- [x] Task: Implement HTTP Client & Bot-Blocking Scanner (`src/checker.rs`) (35a3637)
- [x] Task: Phase Verification & Checkpoint (Refer to workflow.md) (35a3637)

## Phase 3: Recursive Spidering & Concurrency Engine [checkpoint: 7108555]
- [x] Task: Write Unit Tests for Concurrent Visited Tracker & Crawler Engine (7108555)
- [x] Task: Implement Async Crawler Engine with Semaphore Concurrency & Depth Bounds (`src/crawler.rs`) (7108555)
- [x] Task: Phase Verification & Checkpoint (Refer to workflow.md) (7108555)

## Phase 4: CLI Interface & Tabular Output Integration [checkpoint: 7108555]
- [x] Task: Integrate Clap Arguments, Comfy-Table, and Indicatif Spinner (`src/main.rs`) (7108555)
- [x] Task: End-to-End Integration Testing & Phase Verification Checkpoint (Refer to workflow.md) (7108555)

## Phase: Review Fixes
- [x] Task: Apply review suggestions (3b52178)
