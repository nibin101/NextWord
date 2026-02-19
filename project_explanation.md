# AI Autocomplete System Architecture

## System Overview

This autocomplete system uses a 3-tier architecture with offline training, real-time prediction, and VS Code integration:

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           OFFLINE TRAINING PHASE                        │
└─────────────────────────────────────────────────────────────────────────┘
┌─────────────────┐    
│   Training      │    
│   Data          │    Input: Large text corpus (data.txt)
│   (data.txt)    │    - Books, articles, documentation  
│                 │    - Domain-specific text for better results
└─────────┬───────┘    
          │
          ▼
┌─────────────────┐    
│     Trainer     │    Rust Application
│   (main.rs)     │    - Tokenizes text into words
│                 │    - Builds trigram frequency maps: (w1,w2) → w3
│                 │    - Prunes rare occurrences (min_count=2)
│                 │    - Sorts entries for binary search
└─────────┬───────┘    
          │
          ▼
┌─────────────────┐    ┌─────────────────┐
│   model.bin     │    │   vocab.txt     │    Generated Model Files
│ (Binary Model)  │    │ (Vocabulary)    │    - Efficient serialization
│                 │    │                 │    - Fast loading at runtime
└─────────────────┘    └─────────────────┘

┌─────────────────────────────────────────────────────────────────────────┐
│                           RUNTIME PREDICTION PHASE                      │
└─────────────────────────────────────────────────────────────────────────┘
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   VS Code       │    │  Autocomplete   │    │    Model        │
│   Extension     │    │   Server        │    │    Files        │
│  (TypeScript)   │    │   (Rust)        │    │                 │
│                 │    │                 │    │                 │
│  - Registers    │    │  - HTTP API     │    │  - Loaded into  │
│    completion   │    │  - Port 3000    │    │    memory       │
│    provider     │    │  - Binary       │    │  - O(log n)     │
│  - Triggers     │    │    search       │    │    lookups      │
│    on space     │    │  - Top-5        │    │                 │
│  - Sends HTTP   │    │    results      │    │                 │
│    requests     │    │                 │    │                 │
└─────────┬───────┘    └─────────┬───────┘    └─────────────────┘
          │                      │
          │ POST /predict        │
          │ {"context": "..."}   │
          ▼                      ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                          PREDICTION PIPELINE                            │
│                                                                         │
│  Input: "the quick brown"                                               │
│     ↓                                                                   │
│  Extract last 2 words: ["quick", "brown"]                              │
│     ↓                                                                   │
│  Convert to IDs: [1247, 2834]                                          │
│     ↓                                                                   │
│  Binary search for trigrams: (1247, 2834) → [fox, dog, cat, ...]       │
│     ↓                                                                   │
│  Sort by frequency, return top-5                                        │
│     ↓                                                                   │
│  Response: {"suggestions": ["fox", "dog", "cat", "horse", "rabbit"]}    │
└─────────────────────────────────────────────────────────────────────────┘


