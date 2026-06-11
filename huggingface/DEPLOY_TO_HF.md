# Deploy to Hugging Face

The Hugging Face connector is authenticated as `Profrandom`, but HF Jobs upload failed because the account currently has insufficient prepaid credits. Use this file to deploy manually or from a local shell.

## Target repos

```text
https://huggingface.co/spaces/Profrandom/sparkctl-demo
https://huggingface.co/datasets/Profrandom/sparkctl-context-artifacts
```

## Option A — Hugging Face web UI

1. Create a new Space:
   - Owner: `Profrandom`
   - Name: `sparkctl-demo`
   - SDK: `Gradio`
   - Visibility: public
   - License: MIT
2. Upload the contents of:

```text
huggingface/space/
```

3. Create a new Dataset:
   - Owner: `Profrandom`
   - Name: `sparkctl-context-artifacts`
   - Visibility: public
   - License: MIT
4. Upload:

```text
huggingface/dataset/README.md
```

5. Add reviewed artifacts only after confirming they contain no secrets or private trace data.

## Option B — local CLI

Install dependencies:

```bash
pip install -U huggingface_hub
huggingface-cli login
```

Create repos:

```bash
python - <<'PY'
from huggingface_hub import create_repo
create_repo('Profrandom/sparkctl-demo', repo_type='space', space_sdk='gradio', exist_ok=True, private=False)
create_repo('Profrandom/sparkctl-context-artifacts', repo_type='dataset', exist_ok=True, private=False)
PY
```

Upload files:

```bash
python - <<'PY'
from huggingface_hub import HfApi
api = HfApi()
api.upload_folder(
    repo_id='Profrandom/sparkctl-demo',
    repo_type='space',
    folder_path='huggingface/space',
    commit_message='Prepare sparkctl demo Space placeholder',
)
api.upload_folder(
    repo_id='Profrandom/sparkctl-context-artifacts',
    repo_type='dataset',
    folder_path='huggingface/dataset',
    commit_message='Prepare sparkctl context artifacts dataset placeholder',
)
PY
```

## Option C — GitHub Actions sync later

A future workflow can sync `huggingface/space` to the Space using `huggingface/hub-sync` once an `HF_TOKEN` repository secret is configured.

Do not commit tokens or secrets.

## Claim hygiene

Use only fixture-bound wording:

- Agent trace replay consistency: `1.000000`
- Agent operational drift: `0.000000`
- Agent average compression ratio: `1.773954`
- MCP replay evaluation: deterministic, no LLM judges, no external APIs

Avoid:

- compliance claims
- official SPARK compatibility claims
- production-readiness claims
- unrestricted safety claims
- universal compression performance claims
