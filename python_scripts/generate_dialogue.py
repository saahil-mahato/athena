import os
import sys
import json
import random
import google.generativeai as genai

from google.generativeai.protos import Schema, Type
from google.generativeai.types import HarmCategory, HarmBlockThreshold


genai.configure(api_key=os.environ["GEMINI_API_KEY"])


def get_prompt(input_data):
    # Prepare the request to the Gemini API
    prompt = input_data.get("prompt", "")
    max_tokens = input_data.get("max_tokens", 150)
    temperature = input_data.get("temperature", 0.7)

    return { "text": prompt, "maxTokens": max_tokens, "temperature": temperature }


def get_npc_data(input_data):
    # Extract additional context from the input data
    current_state = input_data.get("current_state", "Neutral")
    future_state = input_data.get("future_state", "Neutral")
    current_emotion = input_data.get("current_emotion", "Neutral")
    knowledge_graph = input_data.get("knowledge_graph", {})

    personality = {
        "openness": input_data.get("openness", 0.5),
        "conscientiousness": input_data.get("conscientiousness", 0.5),
        "extraversion": input_data.get("extraversion", 0.5),
        "agreeableness": input_data.get("agreeableness", 0.5),
        "neuroticism": input_data.get("neuroticism", 0.5),
    }

    return { "currentState": current_state, "futureState": future_state,"currentEmotion": current_emotion, "personality": personality, "knowledgeGraph": knowledge_graph }


def get_description(npc_data, instruction):
    return f"""
        The NPC has the following personality. Each attribute in between 0.0 to 1.0 representing intensity: {str(npc_data["personality"])}
        The NPC has the following current emotion: {str(npc_data["currentEmotion"])}
        The NPC has the following current state: {str(npc_data["currentState"])}
        The NPC has the following future state that it is going to: {str(npc_data["futureState"])}
        The NPC has the following knowledge graph: {str(npc_data["knowledgeGraph"])}

        Now, please do the following task with all the above information: f{instruction}
    """


def build_schema_unit(npc_data, instruction):
    schema = Schema(
        type = Type.OBJECT,
        properties = {
            "text": Schema(
                type = Type.STRING,
                description = get_description(npc_data, instruction)
            ),
        },
        required = ["text"]
    )

    return schema


def build_schema(npc_data):
    schema = Schema(
        type=Type.OBJECT,
        properties={
            "npcDialogue": build_schema_unit(npc_data, "Create a witty and engaging dialogue for the NPC that reflects their personality and current emotional state."),
            "pcDialogue": build_schema_unit(npc_data, "Create a response from the PC that is influenced by the NPC's actions and dialogue."),
            "npcEmotions": build_schema_unit(npc_data, "Detail the NPC's emotional state, including any internal conflicts or motivations."),
            "pcEmotions": build_schema_unit(npc_data, "Detail the emotional response of the PC towards the NPC, considering their history and context."),
            "eventNarration": build_schema_unit(npc_data, "Provide a lively description of the current scene and events surrounding the NPC."),
            "contextualHints": build_schema_unit(npc_data, "Include any relevant background information or context that influences the NPC's dialogue and actions."),
            "npcGoals": build_schema_unit(npc_data, "Describe the NPC's current goals or objectives, which may drive their actions and dialogue."),
            "pcGoals": build_schema_unit(npc_data, "Describe the PC's goals or objectives in relation to the NPC, which may create tension or alignment."),
            "npcHistory": build_schema_unit(npc_data, "Provide key details about the NPC's past experiences that shape their personality and current behavior."),
            "pcHistory": build_schema_unit(npc_data, "Provide key details about the PC's past experiences with the NPC or similar situations."),
            "npcRelationships": build_schema_unit(npc_data, "Describe the NPC's relationships with other characters, which may influence their interactions."),
            "pcRelationships": build_schema_unit(npc_data, "Describe the PC's relationships with the NPC and other characters, which may impact their responses."),
            "npcMotivations": build_schema_unit(npc_data, "Explain the underlying motivations that drive the NPC's actions and dialogue, such as desires, fears, or beliefs."),
            "pcMotivations": build_schema_unit(npc_data, "Explain the PC's motivations in relation to the NPC and the current situation.")
        },
        description="Generate immersive and contextually rich dialogue and narrative based on the NPC's character and situation.",
        required=["npcDialogue", "pcDialogue", "npcEmotions", "pcEmotions", "eventNarration", "contextualHints", "npcGoals", "pcGoals", "npcHistory", "pcHistory", "npcRelationships", "pcRelationships", "npcMotivations", "pcMotivations"]
    )

    return schema


def save_response_to_file(data, filename='response.json'):
    """Save the JSON response to a file in the current directory."""
    with open(filename, 'w') as json_file:
        json.dump(data, json_file, indent=4)  # Use indent for pretty formatting
    print(f"Response saved to {filename}")


def main():
    input_json = sys.stdin.read()
    input_data = json.loads(input_json)

    prompt = get_prompt(input_data)

    npc_data = get_npc_data(input_data)

    schema = build_schema(npc_data)


    model = genai.GenerativeModel(
        'gemini-1.5-flash',
        generation_config={"response_mime_type": "application/json", "response_schema": schema},
        system_instruction="You are a game designer and very good at literature. You are designing texts and dialogues for a game"
    )

    response = model.generate_content(
        prompt["text"],
        safety_settings={
            HarmCategory.HARM_CATEGORY_HARASSMENT: HarmBlockThreshold.BLOCK_NONE,
            HarmCategory.HARM_CATEGORY_HATE_SPEECH: HarmBlockThreshold.BLOCK_NONE,
            HarmCategory.HARM_CATEGORY_SEXUALLY_EXPLICIT: HarmBlockThreshold.BLOCK_NONE,
            HarmCategory.HARM_CATEGORY_DANGEROUS_CONTENT: HarmBlockThreshold.BLOCK_NONE
        }
    )

    # Load the response text as JSON
    data = json.loads(response.text)

    # Save the JSON response to a file
    save_response_to_file(data)
    


if __name__ == "__main__":
    main()
