from pydantic_settings import BaseSettings

class Settings(BaseSettings):
    database_url: str
    app_name: str = "kDB API"
    debug: bool = True
    environment: str = "development"
    
    class Config:
        env_file = ".env"

settings = Settings()
