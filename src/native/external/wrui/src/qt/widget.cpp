#include "../../include/wrui.h"

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

void object_setup(GUObject* object, void* data) {
	object->p = data;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

void widget_setup(GUWidget* widget, void* data) {
	widget->object = new GUObject;
	object_setup(widget->object, data);
}
