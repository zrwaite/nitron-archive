from datetime import date
from ariadne import convert_kwargs_to_snake_case
from database import db
from api.models.user import User
from modules.hash import hash_password
from errors import APIError
from .mutation import mutation


@convert_kwargs_to_snake_case
@mutation.field('createUser')
def create_user_resolver(obj, info, data: dict):
    """Create a new user"""
    try:
        prev_user = User.query.filter(User.username == data['username']).scalar()
        if prev_user:
            raise APIError("username in use")
        else:
            user = User(
                username=data['username'],
                hash=hash_password(data['password']),
                email=data['email'],
                created_at=date.today(),
            )
            db.session.add(user)
            db.session.commit()
            return {
                "user": user,
                "token": ""
            }
    except ValueError:
        raise APIError("error creating user")


@convert_kwargs_to_snake_case
@mutation.field('updateUser')
def update_user_resolver(obj, info, data: dict):
    print("hello")
    """Update User"""
    try:
        user = User.query.filter(User.username == data['username']).scalar()
        if user:
            if data['email'] is not None:
                user.email = data['email']
            db.session.add(user)
            db.session.commit()
            return user
        else:
            raise APIError("user not found")
    except AttributeError:
        raise APIError("user not found")


@convert_kwargs_to_snake_case
@mutation.field('deleteUser')
def delete_user_resolver(obj, info, username):
    try:
        user = User.query.get(username)
        db.session.delete(user)
        db.session.commit()
        return True
    except AttributeError:
        raise APIError("user not found")
