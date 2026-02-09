from pydantic_settings import BaseSettings

class Settings(BaseSettings):
    database_url: str = "postgresql://kdbuser:kdbpass@database:5432/kdb"
    app_name: str = "kDB API"
    debug: bool = True
    
    class Config:
        env_file = ".env"

settings = Settings()
