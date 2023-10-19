# cloud-run-with-rust

Rust で実装して Cloud Run にデプロイする

【参考】
[Cloud Run にサービスをデプロイする](https://cloud.google.com/run/docs/quickstarts/build-and-deploy/deploy-service-other-languages?hl=ja)

## Run on local

run

```bash
cargo run
```

GET

```bash
curl -X GET http://localhost:8080
```

POST

```bash
curl -d "@pubsub-push-sample.json" -X POST -H "Content-Type: application/json" http://localhost:8080
```

## Run on Cloud Run

cloud settings

```bash
gcloud services enable cloudbuild.googleapis.com --project={PROJECT_ID} 
gcloud services enable run.googleapis.com --project={PROJECT_ID} 
```

Build & deploy

```bash
gcloud builds submit . --tag asia.gcr.io/{PROJECT_ID}/cloud-run-with-rust --project {PROJECT_ID} 
gcloud run deploy cloud-run-with-rust \
  --project {PROJECT_ID} \
  --image asia.gcr.io/{PROJECT_ID}/cloud-run-with-rust \
  --platform managed \
  --region asia-northeast1 \
  --memory 256Mi \
  --allow-unauthenticated
```

GET

```bash
curl -X GET https://{endpoint}
```

POST

```bash
curl -d "@pubsub-push-sample.json" -X POST -H "Content-Type: application/json" https://{endpoint}
```
