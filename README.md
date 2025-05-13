# The AI-Native Terminal Editor
> An editor where every keystroke can talk to a model—quietly, powerfully, and when you need it most.

led is an LLM-native terminal editor designed from the ground up to integrate AI deeply, not as an add-on—but as a natural part of how you edit, write, refactor, and think.

It’s not just an editor with AI support. It is an AI interface—fast, minimal, scriptable, and chat-native. Built for devs who work in terminals, containers, and over SSH, but want the power of large language models at their fingertips.

## 🌐 Philosophy
- Text is a conversation — Everything you write is context. Everything you see is promptable.
- AI is not a feature — It’s an extension of the editor, always available, never in your way.
- Built for the loop — Write, refine, test, explain, repeat. led amplifies that cycle with LLMs.

## 🔥 Core Capabilities
### ✨ Seamless AI-Native Editing
- Type # fix this bug → led will inline the suggestion
-	Use /explain, /refactor, or /summarize inline in any buffer
-	Multimodel backend: OpenAI, Claude, local (Ollama, LM Studio), or your own gateway

### 💬 Chat-Like Thinking
-	led --ai launches a contextual chat interface right in the buffer
-	Conversation history tied to files or projects
-	Ask “what’s wrong with this function?” and get reasoning grounded in code

### 🧠 Context-Aware Actions
- Works with git state, file tree, or language syntax
- Intelligent chunking and token window management
- Real-time LLM suggestions that adapt to what you’re working on

## 📦 Install
```shell
brew install led
```
Supports macOS, Linux, and WSL

## 🔭 Roadmap
- Context caching for AI offline access
- Project-wide code embeddings + RAG
- Multimodal support (audio prompts, image-to-code)
- Full AI agent integration via natural language
