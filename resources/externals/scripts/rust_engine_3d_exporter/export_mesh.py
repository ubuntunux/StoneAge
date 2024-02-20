from enum import Enum
import datetime
import os
import time
import json

import importlib
import logging
importlib.reload(logging)
from logging.handlers import RotatingFileHandler

import bpy
import bpy_extras


def create_logger(logger_name, log_dirname, level):
    # prepare log directory
    if not os.path.exists(log_dirname):
        os.makedirs(log_dirname)
    log_file_basename = datetime.datetime.fromtimestamp(time.time()).strftime(f'{logger_name}_%Y%m%d_%H%M%S.log')
    log_filename = os.path.join(log_dirname, log_file_basename)

    # create logger
    logger = logging.getLogger(log_dirname)
    logger.setLevel(level=level)

    # add handler
    stream_handler = logging.StreamHandler()
    file_max_byte = 1024 * 1024 * 100 #100MB
    backup_count = 10
    file_handler = logging.handlers.RotatingFileHandler(log_filename, maxBytes=file_max_byte, backupCount=backup_count)

    logger.addHandler(stream_handler)
    logger.addHandler(file_handler)

    # set formatter
    formatter = logging.Formatter(fmt='%(asctime)s,%(msecs)03d [%(levelname)s|%(filename)s:%(lineno)d] %(message)s', datefmt='%Y-%m-%d:%H:%M:%S')
    stream_handler.setFormatter(formatter)
    file_handler.setFormatter(formatter)
    return logger


class ResourceTypeInfo:
    def __init__(self, resource_dirname, resource_ext, external_ext):
        self.resource_dirname = resource_dirname
        self.resource_ext = resource_ext
        self.external_ext = external_ext
        

class ResourceType(Enum):
    MATERIAL_INSTANCE = ResourceTypeInfo('material_instances', '.matinst', None)
    MODEL = ResourceTypeInfo('models', '.model', None)
    MESH = ResourceTypeInfo('meshes', '.mesh', '.gltf')


class RustEngine3DExporter:
    def __init__(self, library_name):
        self.library_name = library_name
        self.asset_library = bpy.context.preferences.filepaths.asset_libraries.get(library_name)
        if self.asset_library:
            log_dirname = os.path.join(self.asset_library.path, '.log')
            self.logger = create_logger(logger_name=library_name, log_dirname=log_dirname, level=logging.DEBUG)
            
    def export_collection(self, collection):
        asset_path = collection.asset_data.catalog_simple_name.split('-')
        if 2 < len(asset_path):
            asset_library_name = asset_path[0]
            asset_type_name = asset_path[1]
            relative_path = '/'.join(asset_path[1:])
            external_path = self.asset_library.path
            resource_path = os.path.split(external_path)[0]
            
            self.logger.info(f'relative_path: {relative_path}, external_path: {external_path}, resource_path: {resource_path}, collection.name: {collection.name}')

            for obj in collection.all_objects:
                self.logger.info(f'{obj.name}: {obj.type}')

            # create collection
            empty = bpy.data.objects.new(collection.name, None)
            empty.instance_type = 'COLLECTION'
            empty.instance_collection = collection
            bpy.context.scene.collection.objects.link(empty)
            
            # TODO: select specify object
            bpy.ops.object.select_all()

            # export resource
            external_filepname = os.path.join(external_path, relative_path, collection.name)
            resource_filename = os.path.join(resource_path, relative_path, collection.name)
            export_filepath = ''
            
            if 'meshes' == asset_type_name:
                export_filepath = external_filepname + '.gltf'
                bpy.ops.export_scene.gltf(
                    filepath=export_filepath,
                    export_format='GLTF_SEPARATE',
                    use_selection=True,
                    export_yup=True,
                    export_texcoords=True,
                    export_normals=True,
                    export_tangents=True,
                    export_colors=True,
                    export_materials='NONE',
                    export_skins=True,
                    export_animations=True,
                    export_force_sampling=True,
                    export_optimize_animation_size=False
                )
            elif 'models' == asset_type_name:
                if 0 < len(collection.children):
                    for mesh_collection in collection.children:
                        mesh_data = mesh_collection.override_library.reference
                        mesh_library_path = mesh_data.asset_data.catalog_simple_name
                        print(f'mesh_library_path: {mesh_library_path}')
                        for child_object in mesh_data.objects:
                            if 'MESH' == child_object.type:                                
                                print(f'child_object: {child_object.name}, material: {child_object.active_material}')
                        for child_object in mesh_collection.objects:
                            if 'MESH' == child_object.type:                                
                                print(f'child_object: {child_object.name}, material_instance: {child_object.active_material}')
            
            self.logger.info(f'Export {asset_type_name}: {export_filepath}')
            
            # remove collection
            bpy.context.scene.collection.objects.unlink(empty)
            bpy.data.objects.remove(empty)
                
    def spawn_asset(self, collection):
        bpy.ops.object.collection_instance_add(collection=collection.name)
        
    def load_blend_file(self, blend_file):
        with bpy.data.libraries.load(blend_file, assets_only=True, link=True) as (data_from, data_to):
            data_to.materials = data_from.materials
            data_to.meshes = data_from.meshes
            data_to.collections = data_from.collections
            data_to.actions = data_from.actions
            data_to.armatures = data_from.armatures
            data_to.object = data_from.objects
            return data_to
        
    def export_blend(self, blend_file):
        data = self.load_blend_file(blend_file)
            
        for (i, collection) in enumerate(data.collections):
            self.export_collection(collection)
            
        return data
    
    def export_resources(self):
        self.logger.info(f'>>> export_resource: {self.asset_library.path}')
        for dirpath, dirnames, filenames in os.walk(self.asset_library.path):
            for filename in filenames:
                if '.blend' == os.path.splitext(filename)[1].lower():
                    data = self.export_blend(os.path.join(dirpath, filename))


def run_export_resources():
    bpy.context.window.cursor_set('WAIT')

    exporter = RustEngine3DExporter('StoneAge')
    exporter.export_blend('/home/ubuntunux/WorkSpace/StoneAge/resources/externals/models/characters/jack.blend')
    #exporter.export_resources()

    # scene = context.scene
    # objects = scene.objects
    # mesh_objects = [ob for ob in objects if ob.type == 'MESH']
    #
    # for mesh in mesh_objects:
    #     for material in mesh.data.materials:
    #         catalog = material.asset_data.catalog_simple_name.replace('-', '/')
    #         relative_filepath = os.path.join(catalog, material.name)
    #         print(relative_filepath)
    #
    #
    # material_instance_data = {
    #     "material_name": "common/render_static_object",
    #     "material_parameters": {
    #         "textureBase": "environments/desert_ground",
    #         "textureMaterial": "common/default_m",
    #         "textureNormal": "common/default_n"
    #     }
    # }
    #
    # model_data = {
    #     "material_instances": [
    #         "environments/cactus"
    #     ],
    #     "mesh": "environments/cactus"
    # }


    #    with open(resource_info.resource_filepath, 'w') as f:
    #        f.write(json.dumps(game_scene_data, sort_keys=True, indent=4))

    bpy.context.window.cursor_set('DEFAULT')
    return {'FINISHED'}


run_export_resources()

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
