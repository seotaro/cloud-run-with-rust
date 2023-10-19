# cloud-run-with-rust

Rust で実装して Cloud Run にデプロイする

【参考】
[Cloud Run にサービスをデプロイする](https://cloud.google.com/run/docs/quickstarts/build-and-deploy/deploy-service-other-languages?hl=ja)

## Run on local

install

```bash
cargo install --path .
```

run

```bash
cloud-run-with-rust
```

```bash
curl http://localhost:8080
```

## Run on Cloud Run

```bash
gcloud services enable cloudbuild.googleapis.com --project={PROJECT_ID} 
gcloud services enable run.googleapis.com --project={PROJECT_ID} 

gcloud builds submit . --tag asia.gcr.io/{PROJECT_ID}/cloud-run-with-rust --project {PROJECT_ID} 
gcloud run deploy cloud-run-with-rust \
  --project {PROJECT_ID} \
  --image asia.gcr.io/{PROJECT_ID}/cloud-run-with-rust \
  --platform managed \
  --region asia-northeast1 \
  --memory 256Mi \
  --allow-unauthenticated
```

```bash
curl https://{endpoint}
```
