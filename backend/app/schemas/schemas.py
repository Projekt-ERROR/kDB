from pydantic import BaseModel
from datetime import datetime
from typing import Optional

class IngredientBase(BaseModel):
    name: str
    quanity: int
    unit: Optional[str] = None

class IngredientCreate(IngredientBase):
    pass

class Ingredient(IngredientBase):
    id: int
    created_at: datetime
    updated_at: datetime

    class Config:
        from_attributes = True

