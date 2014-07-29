#pragma once

#include <PDUI.h>

struct PDReader;
struct PDWriter;
struct PDViewPlugin;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

namespace prodbg
{

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct ViewPluginInstance
{
	PDUI ui;
	PDViewPlugin* plugin;
	void* userData;
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

ViewPluginInstance* PluginInstance_createViewPlugin();
bool PluginInstance_init(ViewPluginInstance* instance);

}
