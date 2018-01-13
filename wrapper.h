#include <interface/mmal/mmal.h>
#include <interface/mmal/util/mmal_default_components.h>
#include <interface/mmal/util/mmal_connection.h>
#include <interface/mmal/util/mmal_util.h>
#include <interface/mmal/util/mmal_util_params.h>
#include <interface/mmal/mmal_parameters_camera.h>
#include <interface/mmal/mmal_encodings.h>
#include <interface/mmal/mmal_events.h>
#include <interface/vcos/vcos_types.h>

// These are hacks to make the linker link
#include <bcm_host.h>
#include <interface/mmal/vc/mmal_vc_api.h>
#include <interface/vcos/vcos.h>
