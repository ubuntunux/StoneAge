from enum import Enum
import os
import json

import bpy
import bpy_extras


class ResouerceTypeInfo:
    def __init__(self, resource_dirname, resource_ext, external_ext):
        self.resource_dirname = resource_dirname
        self.resource_ext = resource_ext
        self.external_ext = external_ext
        

class ResourceType(Enum):
    MODEL = ResouerceTypeInfo('models', '.model', '.gltf')
    MESH = ResouerceTypeInfo('meshes', '.mesh', '.gltf')
    
    
class ResourceInfo:
    def __init__(self, resource_type, filepath):
        # initialize
        self.resource_type = resource_type
        self.filepath = filepath
        self.external_filepath = ''
        self.resource_name = ''
        self.resource_filepath = ''
        
        resource_type_info = resource_type.value
        
        # extract filename
        dirname, filename = os.path.split(filepath)
        basename = os.path.splitext(filename)[0]
        external_filename = filename
        resource_filename = filename
        external_file_basename = basename + resource_type_info.external_ext
        resource_file_basename = basename + resource_type_info.resource_ext
            
        # external filepath
        self.external_filepath = os.path.join(dirname, external_file_basename)
        
        # extract resource name, filepath
        while True:
            head, tail = os.path.split(dirname)
            if '' == tail or resource_type_info.resource_dirname == tail:
                resource_dirname = resource_filepath = os.path.split(head)[0]
                relative_dirname = os.path.split(os.path.relpath(filepath, head))[0]                
                self.resource_name = os.path.join(relative_dirname, basename)
                self.resource_filepath = os.path.join(resource_dirname, os.path.join(relative_dirname, resource_file_basename))
                break
            dirname = head


def save(operator, context, filepath='', **keywords):
    if filepath == '':
        return {'FINISHED'}
    
    resource_info = ResourceInfo(ResourceType.MESH, filepath)
    
    context.window.cursor_set('WAIT')

    scene = context.scene
    objects = scene.objects
    mesh_objects = [ob for ob in objects if ob.type == 'MESH']
    
    for mesh in mesh_objects:
        for material in mesh.data.materials:
            catalog = material.asset_data.catalog_simple_name.replace('-', '/')
            relative_filepath = os.path.join(catalog, material.name)
            print(relative_filepath)
            

    material_instance_data = {
        "material_name": "common/render_static_object",
        "material_parameters": {
            "textureBase": "environments/desert_ground",
            "textureMaterial": "common/default_m",
            "textureNormal": "common/default_n"
        }
    }
    
    model_data = {
        "material_instances": [
            "environments/cactus"
        ], 
        "mesh": "environments/cactus"
    }


#    with open(resource_info.resource_filepath, 'w') as f:
#        f.write(json.dumps(game_scene_data, sort_keys=True, indent=4))

    context.window.cursor_set('DEFAULT')
    return {'FINISHED'}



save(None, bpy.context, filepath=bpy.data.filepath)


''' 
# model
>>> cactus = bpy.context.selected_objects[0]
>>> cactus.instance_collection.library.filepath
'//../models/environments/cactus.blend'

>>> cactus.instance_collection.id_data.asset_data.catalog_simple_name
'StoneAge-models-environments'


# mesh
>>> mesh = cactus.instance_collection.objects[0]
>>> mesh.data.library.filepath
'//../meshes/environments/cactus.blend'



# material
>>> material = mesh.data.materials[0]
>>> material.library.filepath
'//../materials/common/render_static_object.blend'

>>> material.asset_data.catalog_simple_name
'materials-common'


# material instance
>>> material_instance = mesh.material_slots[0].material
>>> material_instance.library.filepath
'//../models/environments/desert_ground.blend'

>>> material_instance.asset_data.catalog_simple_name
'StoneAge-material_instances-environments'

'''
