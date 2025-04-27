CMSIS_ROOT = __CMSIS_ROOT__

# Compilation tools
CC := clang
AR := llvm-ar

# Compilation flags (here for Cortex-M7)
CFLAGS := -mcpu=__MCPU__ --target=__TARGET__ \
 -Wsign-compare \
 -Wdouble-promotion \
 -O3 -ffast-math \
 -DNDEBUG \
 -Wall -Wextra  -Werror \
 -fshort-enums -fshort-wchar \
 -mfloat-abi=hard

# Path to CMSIS_CORE
CMSIS_CORE := $(CMSIS_ROOT)/CMSIS_CORE

# Path to CMSIS_DSP
CMSIS_DSP := $(CMSIS_ROOT)/CMSIS_DSP

# Path to CMSIS Core includes for Cortex-M
# For low end Cortex-A, use Core_A
# For high end Cortex-A (aarch64), don't use CMSIS 
# Core Includes (Refer to the CMSIS-DSP README to 
# know how to build in that case)
CMSIS_CORE_INCLUDES := $(CMSIS_CORE)/CMSIS/Core/Include 

# Sources
SRCS := $(CMSIS_DSP)/Source/BasicMathFunctions/BasicMathFunctions.c \
 $(CMSIS_DSP)/Source/CommonTables/CommonTables.c \
 $(CMSIS_DSP)/Source/InterpolationFunctions/InterpolationFunctions.c \
 $(CMSIS_DSP)/Source/BayesFunctions/BayesFunctions.c \
 $(CMSIS_DSP)/Source/MatrixFunctions/MatrixFunctions.c \
 $(CMSIS_DSP)/Source/ComplexMathFunctions/ComplexMathFunctions.c \
 $(CMSIS_DSP)/Source/QuaternionMathFunctions/QuaternionMathFunctions.c \
 $(CMSIS_DSP)/Source/ControllerFunctions/ControllerFunctions.c \
 $(CMSIS_DSP)/Source/SVMFunctions/SVMFunctions.c \
 $(CMSIS_DSP)/Source/DistanceFunctions/DistanceFunctions.c \
 $(CMSIS_DSP)/Source/StatisticsFunctions/StatisticsFunctions.c \
 $(CMSIS_DSP)/Source/FastMathFunctions/FastMathFunctions.c \
 $(CMSIS_DSP)/Source/SupportFunctions/SupportFunctions.c \
 $(CMSIS_DSP)/Source/FilteringFunctions/FilteringFunctions.c \
 $(CMSIS_DSP)/Source/TransformFunctions/TransformFunctions.c \
 $(CMSIS_DSP)/Source/WindowFunctions/WindowFunctions.c

# Includes
DSP_INCLUDES := $(CMSIS_DSP)/Include \
  $(CMSIS_DSP)/PrivateInclude 

# If Neon and Cortex-A
#DSP_INCLUDES += $(CMSIS_DSP)/ComputeLibrary/Include 
#SRCS += $(CMSIS_DSP)/ComputeLibrary/Source/arm_cl_tables.c 

# Compilation flags for include folders
INC_FLAGS := $(addprefix -I,$(DSP_INCLUDES))
INC_FLAGS += $(addprefix -I,$(CMSIS_CORE_INCLUDES))
CFLAGS += $(INC_FLAGS) -I/usr/include

# Output folder for build products
BUILDDIR := $(CMSIS_ROOT)/builddir

OBJECTS := $(SRCS:%=$(BUILDDIR)/%.o)

# Build rules
$(BUILDDIR)/libCMSISDSP.a: $(OBJECTS)
	$(AR) -rc $@ $(OBJECTS)
	

$(BUILDDIR)/%.c.o: %.c
	mkdir -p $(dir $@)
	$(CC) -static -c $(CFLAGS) $(CPPFLAGS) $< -o $@
