from fastapi import FastAPI

app = FastAPI(title="kDB API")

@app.get("/")
def root():
    return {"message": "kDB API is running!"}

@app.get("/health")
def health_check():
    return {"status": "healthy", "database": "not connected yet"}
