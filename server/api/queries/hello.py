from .query import query


@query.field('hello')
def hello_world(*_):
    """Hello World"""
    return "World!"
