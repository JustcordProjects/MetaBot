.PHONY: static upload

EXE_NAME ?= metabot
TARGET   ?= x86_64-unknown-linux-musl
PROFILE  ?= release

NEED_UPLOAD := false
ifeq ($(MAKECMDGOALS),)
    NEED_UPLOAD := true
else ifneq ($(filter all upload,$(MAKECMDGOALS)),)
    NEED_UPLOAD := true
endif

ifeq ($(NEED_UPLOAD),true)
    ifeq ($(UPLOAD_USER),)
        $(error "environment variable 'UPLOAD_USER' must be set")
    endif
    ifeq ($(UPLOAD_HOST),)
        $(error "environment variable 'UPLOAD_HOST' must be set")
    endif
    ifeq ($(UPLOAD_PATH),)
        $(error "environment variable 'UPLOAD_PATH' must be set")
    endif
endif

CARGO ?= cargo

BUILD_DIR := target/$(TARGET)/$(PROFILE)
EXE := $(BUILD_DIR)/$(EXE_NAME)

.PHONY: all static upload clean
all: static

static:
	$(CARGO) build --$(PROFILE) --target $(TARGET)

clean:
	$(CARGO) clean

upload: static
	scp $(SCP_FLAGS) $(EXE) \
		$(UPLOAD_USER)@$(UPLOAD_HOST):$(UPLOAD_PATH)
