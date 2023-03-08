import json

# file = open(r"C:\Users\dhruv\CODE\Nitron\AI work\config.json", 'w')

# data = {}
# data['emotions'] = ['ecstasy', 'admiration', 'terror', 'amazement', 'grief', 'loathing', 'rage', 'vigilance']

# json.dump(data, file)

file = open(r"C:\Users\dhruv\CODE\Nitron\AI work\world.json", 'w')

data = {}

#create all objects

objects = {}

objects['gun'] = [0, 2, 8, 3, 1, 4, 3, 5]
objects['rose'] = [7, 8, 0, 1, 0, 0, 0, 2]
objects['medKit'] = [4, 3, 0, 1, 0, 0, 0, 1]
objects['letter'] = [0, 0, 3, 0, 6, 1, 3, 1]
objects['default'] = [0, 0, 0, 0, 0, 0, 0, 0]

data['objects'] = objects
data['object_classification'] = {'gun': "weapon",  'rose': "gift", 'medKit':'utility', 'letter':'gift', 'default': 'None'}

actions = {}
actions['attack'] = [1, 0, 7, 0, 0, 5, 9, 4]
actions['offer'] = [7, 4, 1, 1, 0, 0, 0, 2]
actions['use'] = [1, 0, 0, 0, 0, 0, 0, 1]

data['actions'] = actions

json.dump(data, file)
