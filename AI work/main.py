import run_model
import cosine_similarity
import json

file = open("AI work\world.json")
world_data = json.load(file)

def get_response(object, actions):
    print("actions: " + actions)
    print("object: " + object)
    if object not in world_data['objects'].keys():
        print("ERROR, object not in world")
    object = world_data['objects'][object]
    response = run_model.get_response(object, actions)
    reaction = cosine_similarity.find_best_match(response, [reaction for reaction in world_data['reactions'].values()])
    reaction = [key for key in world_data['reactions'].keys()][reaction['best_match']]
    return reaction

print("reaction: " + get_response("gun", "use"))