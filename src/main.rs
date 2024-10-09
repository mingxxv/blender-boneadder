use std::io::{self, Write};
use arboard::Clipboard;
use crossterm::{terminal, event::{self, Event, KeyCode}};

fn main() {
    // Ask for parent bone name
    let parent_bone_name = ask_input("Enter the parent bone name: ");

    // Ask for new bone name
    let new_bone_name = ask_input("Enter the new bone name: ");

    // Generate Python script with placeholders
    let python_script = generate_python_script(&parent_bone_name, &new_bone_name);

    // Copy the script to the clipboard
    if let Err(e) = copy_to_clipboard(&python_script) {
        eprintln!("Error copying to clipboard: {}", e);
    } else {
        println!("Python script copied to clipboard.");
    }

    // Notify the user
    println!("Press any key to exit...");

    // Wait for any key press
    wait_for_keypress();

    // Exit
    println!("Exiting...");
}

// Function to ask for user input
fn ask_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

// Function to generate the Python script with the provided bone names
fn generate_python_script(parent_bone_name: &str, new_bone_name: &str) -> String {
    format!(
r#"import bpy

def add_bone_to_armature(armature_name, parent_bone_name, new_bone_name):
    # Find the armature object
    armature = bpy.data.objects.get(armature_name)
    
    if armature is None or armature.type != 'ARMATURE':
        print(f"Armature '{{}}' not found or is not an armature.".format(armature_name))
        return

    # Enter Edit Mode
    bpy.context.view_layer.objects.active = armature
    bpy.ops.object.mode_set(mode='EDIT')

    # Get the armature's edit bones
    edit_bones = armature.data.edit_bones
    
    # Check if the parent bone exists
    parent_bone = edit_bones.get(parent_bone_name)
    if parent_bone is None:
        print(f"Parent bone '{{}}' not found.".format(parent_bone_name))
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
parent_bone_name = "{parent_bone_name}"
new_bone_name = "{new_bone_name}"

# Add the new bone to the armature
add_bone_to_armature(armature_name, parent_bone_name, new_bone_name)
"#)
}

// Function to copy the script to the clipboard using arboard
fn copy_to_clipboard(script: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(script.to_string())?;
    Ok(())
}

// Function to wait for any key press to exit
fn wait_for_keypress() {
    terminal::enable_raw_mode().unwrap();
    loop {
        if let Ok(Event::Key(event)) = event::read() {
            if event.code != KeyCode::Null {
                break;
            }
        }
    }
    terminal::disable_raw_mode().unwrap();
}
