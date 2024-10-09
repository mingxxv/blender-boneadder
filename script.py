import bpy

def add_bone_to_armature(armature_name, parent_bone_name, new_bone_name):
    # Find the armature object
    armature = bpy.data.objects.get(armature_name)
    
    if armature is None or armature.type != 'ARMATURE':
        print(f"Armature '{armature_name}' not found or is not an armature.")
        return

    # Enter Edit Mode
    bpy.context.view_layer.objects.active = armature
    bpy.ops.object.mode_set(mode='EDIT')

    # Get the armature's edit bones
    edit_bones = armature.data.edit_bones
    
    # Check if the parent bone exists
    parent_bone = edit_bones.get(parent_bone_name)
    if parent_bone is None:
        print(f"Parent bone '{parent_bone_name}' not found.")
        bpy.ops.object.mode_set(mode='OBJECT')
        return

    # Create a new bone
    new_bone = edit_bones.new(new_bone_name)
    
    # Set the head and tail of the new bone (relative to the parent bone)
    new_bone.head = parent_bone.tail  # Start where the parent bone ends
    new_bone.tail = (new_bone.head[0], new_bone.head[1], new_bone.head[2] + 0.2)  # Slightly offset in Z-axis
    
    # Parent the new bone to the existing parent bone
    new_bone.parent = parent_bone

    # Exit Edit Mode back to Object Mode
    bpy.ops.object.mode_set(mode='OBJECT')

# Set the name of the armature, parent bone, and new bone
armature_name = "Armature"  # Change to your armature's name
parent_bone_name = ""
new_bone_name = ""

# Add the new bone to the armature
add_bone_to_armature(armature_name, parent_bone_name, new_bone_name)
