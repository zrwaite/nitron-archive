# pylint: disable=unused-argument
from ariadne import convert_kwargs_to_snake_case #ObjectType
from api.models.user import User
from database import db
from errors import APIError
from .query import query

@convert_kwargs_to_snake_case
@query.field('getEmotions')
def get_emotions_resolver(obj, info, text):
    return {
		"angry": 0.0,
		"fear": 0.22,
		"happy": 0.11,
		"sad": 0.22,
		"surprise": 0.44
	}