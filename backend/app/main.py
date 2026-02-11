from fastapi import FastAPI, Depends
from sqlalchemy.orm import Session
from app.database.database import engine, get_db, Base
from app.models import models
from app.schemas import schemas

Base.metadata.create_all(bind=engine)

app = FastAPI(title="kDB API")

@app.get("/")
def root():
    return {"message": "kDB API is running!"}

@app.get("/health")
def health_check():
    try:
        db.execute("SELECT 1")
        return {"status": "healthy", "database": "connected"}
    except Exception as e:
        return {"status": "unhealthy", "database": f"error: {str(e)}"}



