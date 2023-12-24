from bpy_extras.io_utils import (
    ImportHelper,
    ExportHelper,
    orientation_helper,
    axis_conversion,
)
from bpy.props import (
    BoolProperty,
    EnumProperty,
    FloatProperty,
    StringProperty,
)
import bpy
bl_info = {
    "name": "RustEngine3D Game Scene Exporter",
    "author": "ubuntunux@gmail.com",
    "version": (1, 0, 0),
    "blender": (4, 0, 0),
    "location": "File > Import-Export",
    "description": "Game Scene Export objects",
    "warning": "Images must be in file folder, "
               "filenames are limited to DOS 8.3 format",
    "doc_url": "https://github.com/ubuntunux/StoneAge",
    "category": "Import-Export",
}

if "bpy" in locals():
    import importlib
    if "export_game_scene" in locals():
        importlib.reload(export_game_scene)


@orientation_helper(axis_forward='Y', axis_up='Z')
class Export3DS(bpy.types.Operator, ExportHelper):
    """Export to 3DS file format (.game_scene)"""
    bl_idname = "export_scene.rust_engine_3d_game_scene"
    bl_label = 'Export RustEngine3D Game Scene'
    bl_options = {'PRESET', 'UNDO'}

    filename_ext = ".game_scene"
    filter_glob: StringProperty(
        default="*.3ds",
        options={'HIDDEN'},
    )

    scale_factor: FloatProperty(
        name="Scale Factor",
        description="Master scale factor for all objects",
        min=0.0, max=100000.0,
        soft_min=0.0, soft_max=100000.0,
        default=1.0,
    )
    use_scene_unit: BoolProperty(
        name="Scene Units",
        description="Take the scene unit length settings into account",
        default=False,
    )
    use_selection: BoolProperty(
        name="Selection",
        description="Export selected objects only",
        default=False,
    )
    object_filter: EnumProperty(
        name="Object Filter", options={'ENUM_FLAG'},
        items=(('WORLD', "World".rjust(11), "", 'WORLD_DATA',0x1),
               ('MESH', "Mesh".rjust(11), "", 'MESH_DATA', 0x2),
               ('LIGHT', "Light".rjust(12), "", 'LIGHT_DATA',0x4),
               ('CAMERA', "Camera".rjust(11), "", 'CAMERA_DATA',0x8),
               ('EMPTY', "Empty".rjust(11), "", 'EMPTY_AXIS',0x10),
               ),
        description="Object types to export",
        default={'WORLD', 'MESH', 'LIGHT', 'CAMERA', 'EMPTY'},
    )
    use_hierarchy: BoolProperty(
        name="Hierarchy",
        description="Export hierarchy chunks",
        default=False,
    )
    use_keyframes: BoolProperty(
        name="Animation",
        description="Write the keyframe data",
        default=False,
    )
    use_cursor: BoolProperty(
        name="Cursor Origin",
        description="Save the 3D cursor location",
        default=False,
    )

    def execute(self, context):
        from . import export_game_scene

        keywords = self.as_keywords(ignore=("axis_forward",
                                            "axis_up",
                                            "filter_glob",
                                            "check_existing",
                                            ))
        global_matrix = axis_conversion(to_forward=self.axis_forward,
                                        to_up=self.axis_up,
                                        ).to_4x4()
        keywords["global_matrix"] = global_matrix

        return export_3ds.save(self, context, **keywords)

    def draw(self, context):
        pass


class MAX3DS_PT_export_include(bpy.types.Panel):
    bl_space_type = 'FILE_BROWSER'
    bl_region_type = 'TOOL_PROPS'
    bl_label = "Include"
    bl_parent_id = "FILE_PT_operator"

    @classmethod
    def poll(cls, context):
        sfile = context.space_data
        operator = sfile.active_operator

        return operator.bl_idname == "EXPORT_SCENE_OT_max3ds"

    def draw(self, context):
        layout = self.layout
        layout.use_property_split = True
        layout.use_property_decorate = True

        sfile = context.space_data
        operator = sfile.active_operator

        layrow = layout.row(align=True)
        layrow.prop(operator, "use_selection")
        layrow.label(text="", icon='RESTRICT_SELECT_OFF' if operator.use_selection else 'RESTRICT_SELECT_ON')
        layout.column().prop(operator, "object_filter")
        layrow = layout.row(align=True)
        layrow.prop(operator, "use_hierarchy")
        layrow.label(text="", icon='OUTLINER' if operator.use_hierarchy else 'CON_CHILDOF')
        layrow = layout.row(align=True)
        layrow.prop(operator, "use_keyframes")
        layrow.label(text="", icon='ANIM' if operator.use_keyframes else 'DECORATE_DRIVER')
        layrow = layout.row(align=True)
        layrow.prop(operator, "use_cursor")
        layrow.label(text="", icon='PIVOT_CURSOR' if operator.use_cursor else 'CURSOR')


class MAX3DS_PT_export_transform(bpy.types.Panel):
    bl_space_type = 'FILE_BROWSER'
    bl_region_type = 'TOOL_PROPS'
    bl_label = "Transform"
    bl_parent_id = "FILE_PT_operator"

    @classmethod
    def poll(cls, context):
        sfile = context.space_data
        operator = sfile.active_operator

        return operator.bl_idname == "EXPORT_SCENE_OT_max3ds"

    def draw(self, context):
        layout = self.layout
        layout.use_property_split = True
        layout.use_property_decorate = False

        sfile = context.space_data
        operator = sfile.active_operator

        layout.prop(operator, "scale_factor")
        layrow = layout.row(align=True)
        layrow.prop(operator, "use_scene_unit")
        layrow.label(text="", icon='EMPTY_ARROWS' if operator.use_scene_unit else 'EMPTY_DATA')
        layout.prop(operator, "axis_forward")
        layout.prop(operator, "axis_up")


# Add to a menu
def menu_func_export(self, context):
    self.layout.operator(Export3DS.bl_idname, text="Game Scene (.game_scene)")


def register():
    bpy.utils.register_class(Export3DS)
    bpy.utils.register_class(MAX3DS_PT_export_include)
    bpy.utils.register_class(MAX3DS_PT_export_transform)
    bpy.types.TOPBAR_MT_file_export.append(menu_func_export)


def unregister():
    bpy.utils.unregister_class(Export3DS)
    bpy.utils.unregister_class(MAX3DS_PT_export_include)
    bpy.utils.unregister_class(MAX3DS_PT_export_transform)
    bpy.types.TOPBAR_MT_file_export.remove(menu_func_export)


if __name__ == "__main__":
    register()
