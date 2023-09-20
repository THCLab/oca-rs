#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
typedef struct _Dart_Handle* Dart_Handle;

typedef struct DartCObject DartCObject;

typedef int64_t DartPort;

typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);

typedef struct wire_uint_8_list {
  uint8_t *ptr;
  int32_t len;
} wire_uint_8_list;

typedef struct wire_MutexOcaBoxRaw {
  const void *ptr;
} wire_MutexOcaBoxRaw;

typedef struct wire_OcaBox {
  struct wire_MutexOcaBoxRaw field0;
} wire_OcaBox;

typedef struct wire_MutexOcaAttrRaw {
  const void *ptr;
} wire_MutexOcaAttrRaw;

typedef struct wire_OcaAttr {
  struct wire_MutexOcaAttrRaw field0;
} wire_OcaAttr;

typedef struct wire_StringList {
  struct wire_uint_8_list **ptr;
  int32_t len;
} wire_StringList;

typedef struct wire_MutexStringMap {
  const void *ptr;
} wire_MutexStringMap;

typedef struct wire_OcaMap {
  struct wire_MutexStringMap field0;
} wire_OcaMap;

typedef struct wire_MutexOcaBundleRaw {
  const void *ptr;
} wire_MutexOcaBundleRaw;

typedef struct wire_OcaBundle {
  struct wire_MutexOcaBundleRaw field0;
} wire_OcaBundle;

typedef struct wire_MutexOcaCaptureBaseRaw {
  const void *ptr;
} wire_MutexOcaCaptureBaseRaw;

typedef struct wire_OcaCaptureBase {
  struct wire_MutexOcaCaptureBaseRaw field0;
} wire_OcaCaptureBase;

typedef struct DartCObject *WireSyncReturn;

void store_dart_post_cobject(DartPostCObjectFnType ptr);

Dart_Handle get_dart_object(uintptr_t ptr);

void drop_dart_object(uintptr_t ptr);

uintptr_t new_dart_opaque(Dart_Handle handle);

intptr_t init_frb_dart_api_dl(void *obj);

void wire_load_oca(int64_t port_, struct wire_uint_8_list *json);

void wire_new__static_method__OcaBox(int64_t port_);

void wire_add_meta__method__OcaBox(int64_t port_,
                                   struct wire_OcaBox *that,
                                   struct wire_uint_8_list *lang,
                                   struct wire_uint_8_list *name,
                                   struct wire_uint_8_list *value);

void wire_add_attribute__method__OcaBox(int64_t port_,
                                        struct wire_OcaBox *that,
                                        struct wire_OcaAttr *attr);

void wire_generate_bundle__method__OcaBox(int64_t port_, struct wire_OcaBox *that);

void wire_add_form_layout__method__OcaBox(int64_t port_,
                                          struct wire_OcaBox *that,
                                          struct wire_uint_8_list *layout);

void wire_add_credential_layout__method__OcaBox(int64_t port_,
                                                struct wire_OcaBox *that,
                                                struct wire_uint_8_list *layout);

void wire_new__static_method__OcaAttr(int64_t port_, struct wire_uint_8_list *name);

void wire_set_attribute_type__method__OcaAttr(int64_t port_,
                                              struct wire_OcaAttr *that,
                                              int32_t attr_type);

void wire_set_flagged__method__OcaAttr(int64_t port_, struct wire_OcaAttr *that);

void wire_set_encoding__method__OcaAttr(int64_t port_, struct wire_OcaAttr *that, int32_t encoding);

void wire_set_cardinality__method__OcaAttr(int64_t port_,
                                           struct wire_OcaAttr *that,
                                           struct wire_uint_8_list *cardinality);

void wire_set_conformance__method__OcaAttr(int64_t port_,
                                           struct wire_OcaAttr *that,
                                           struct wire_uint_8_list *conformance);

void wire_set_label__method__OcaAttr(int64_t port_,
                                     struct wire_OcaAttr *that,
                                     struct wire_uint_8_list *lang,
                                     struct wire_uint_8_list *label);

void wire_set_information__method__OcaAttr(int64_t port_,
                                           struct wire_OcaAttr *that,
                                           struct wire_uint_8_list *lang,
                                           struct wire_uint_8_list *information);

void wire_set_entry_codes__method__OcaAttr(int64_t port_,
                                           struct wire_OcaAttr *that,
                                           struct wire_StringList *entry_codes);

void wire_set_entry_codes_sai__method__OcaAttr(int64_t port_,
                                               struct wire_OcaAttr *that,
                                               struct wire_uint_8_list *sai);

void wire_set_entry__method__OcaAttr(int64_t port_,
                                     struct wire_OcaAttr *that,
                                     struct wire_uint_8_list *lang,
                                     struct wire_OcaMap *entries);

void wire_set_unit_metric__method__OcaAttr(int64_t port_, struct wire_OcaAttr *that, int32_t unit);

void wire_set_unit_imperial__method__OcaAttr(int64_t port_,
                                             struct wire_OcaAttr *that,
                                             int32_t unit);

void wire_set_format__method__OcaAttr(int64_t port_,
                                      struct wire_OcaAttr *that,
                                      struct wire_uint_8_list *format);

void wire_to_json__method__OcaBundle(int64_t port_, struct wire_OcaBundle *that);

void wire_said__method__OcaBundle(int64_t port_, struct wire_OcaBundle *that);

void wire_capture_base__method__OcaBundle(int64_t port_, struct wire_OcaBundle *that);

void wire_overlays__method__OcaBundle(int64_t port_, struct wire_OcaBundle *that);

void wire_attributes__method__OcaCaptureBase(int64_t port_, struct wire_OcaCaptureBase *that);

void wire_flagged_attributes__method__OcaCaptureBase(int64_t port_,
                                                     struct wire_OcaCaptureBase *that);

void wire_new__static_method__OcaMap(int64_t port_);

void wire_insert__method__OcaMap(int64_t port_,
                                 struct wire_OcaMap *that,
                                 struct wire_uint_8_list *key,
                                 struct wire_uint_8_list *value);

void wire_get__method__OcaMap(int64_t port_,
                              struct wire_OcaMap *that,
                              struct wire_uint_8_list *key);

void wire_remove__method__OcaMap(int64_t port_,
                                 struct wire_OcaMap *that,
                                 struct wire_uint_8_list *key);

void wire_get_keys__method__OcaMap(int64_t port_, struct wire_OcaMap *that);

struct wire_MutexOcaAttrRaw new_MutexOcaAttrRaw(void);

struct wire_MutexOcaBoxRaw new_MutexOcaBoxRaw(void);

struct wire_MutexOcaBundleRaw new_MutexOcaBundleRaw(void);

struct wire_MutexOcaCaptureBaseRaw new_MutexOcaCaptureBaseRaw(void);

struct wire_MutexStringMap new_MutexStringMap(void);

struct wire_StringList *new_StringList_0(int32_t len);

struct wire_OcaAttr *new_box_autoadd_oca_attr_0(void);

struct wire_OcaBox *new_box_autoadd_oca_box_0(void);

struct wire_OcaBundle *new_box_autoadd_oca_bundle_0(void);

struct wire_OcaCaptureBase *new_box_autoadd_oca_capture_base_0(void);

struct wire_OcaMap *new_box_autoadd_oca_map_0(void);

struct wire_uint_8_list *new_uint_8_list_0(int32_t len);

void drop_opaque_MutexDynOverlay(const void *ptr);

const void *share_opaque_MutexDynOverlay(const void *ptr);

void drop_opaque_MutexOcaAttrRaw(const void *ptr);

const void *share_opaque_MutexOcaAttrRaw(const void *ptr);

void drop_opaque_MutexOcaBoxRaw(const void *ptr);

const void *share_opaque_MutexOcaBoxRaw(const void *ptr);

void drop_opaque_MutexOcaBundleRaw(const void *ptr);

const void *share_opaque_MutexOcaBundleRaw(const void *ptr);

void drop_opaque_MutexOcaCaptureBaseRaw(const void *ptr);

const void *share_opaque_MutexOcaCaptureBaseRaw(const void *ptr);

void drop_opaque_MutexStringMap(const void *ptr);

const void *share_opaque_MutexStringMap(const void *ptr);

void free_WireSyncReturn(WireSyncReturn ptr);

static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) wire_load_oca);
    dummy_var ^= ((int64_t) (void*) wire_new__static_method__OcaBox);
    dummy_var ^= ((int64_t) (void*) wire_add_meta__method__OcaBox);
    dummy_var ^= ((int64_t) (void*) wire_add_attribute__method__OcaBox);
    dummy_var ^= ((int64_t) (void*) wire_generate_bundle__method__OcaBox);
    dummy_var ^= ((int64_t) (void*) wire_add_form_layout__method__OcaBox);
    dummy_var ^= ((int64_t) (void*) wire_add_credential_layout__method__OcaBox);
    dummy_var ^= ((int64_t) (void*) wire_new__static_method__OcaAttr);
    dummy_var ^= ((int64_t) (void*) wire_set_attribute_type__method__OcaAttr);
    dummy_var ^= ((int64_t) (void*) wire_set_flagged__method__OcaAttr);
    dummy_var ^= ((int64_t) (void*) wire_set_encoding__method__OcaAttr);
    dummy_var ^= ((int64_t) (void*) wire_set_cardinality__method__OcaAttr);
    dummy_var ^= ((int64_t) (void*) wire_set_conformance__method__OcaAttr);
    dummy_var ^= ((int64_t) (void*) wire_set_label__method__OcaAttr);
    dummy_var ^= ((int64_t) (void*) wire_set_information__method__OcaAttr);
    dummy_var ^= ((int64_t) (void*) wire_set_entry_codes__method__OcaAttr);
    dummy_var ^= ((int64_t) (void*) wire_set_entry_codes_sai__method__OcaAttr);
    dummy_var ^= ((int64_t) (void*) wire_set_entry__method__OcaAttr);
    dummy_var ^= ((int64_t) (void*) wire_set_unit_metric__method__OcaAttr);
    dummy_var ^= ((int64_t) (void*) wire_set_unit_imperial__method__OcaAttr);
    dummy_var ^= ((int64_t) (void*) wire_set_format__method__OcaAttr);
    dummy_var ^= ((int64_t) (void*) wire_to_json__method__OcaBundle);
    dummy_var ^= ((int64_t) (void*) wire_said__method__OcaBundle);
    dummy_var ^= ((int64_t) (void*) wire_capture_base__method__OcaBundle);
    dummy_var ^= ((int64_t) (void*) wire_overlays__method__OcaBundle);
    dummy_var ^= ((int64_t) (void*) wire_attributes__method__OcaCaptureBase);
    dummy_var ^= ((int64_t) (void*) wire_flagged_attributes__method__OcaCaptureBase);
    dummy_var ^= ((int64_t) (void*) wire_new__static_method__OcaMap);
    dummy_var ^= ((int64_t) (void*) wire_insert__method__OcaMap);
    dummy_var ^= ((int64_t) (void*) wire_get__method__OcaMap);
    dummy_var ^= ((int64_t) (void*) wire_remove__method__OcaMap);
    dummy_var ^= ((int64_t) (void*) wire_get_keys__method__OcaMap);
    dummy_var ^= ((int64_t) (void*) new_MutexOcaAttrRaw);
    dummy_var ^= ((int64_t) (void*) new_MutexOcaBoxRaw);
    dummy_var ^= ((int64_t) (void*) new_MutexOcaBundleRaw);
    dummy_var ^= ((int64_t) (void*) new_MutexOcaCaptureBaseRaw);
    dummy_var ^= ((int64_t) (void*) new_MutexStringMap);
    dummy_var ^= ((int64_t) (void*) new_StringList_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_oca_attr_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_oca_box_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_oca_bundle_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_oca_capture_base_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_oca_map_0);
    dummy_var ^= ((int64_t) (void*) new_uint_8_list_0);
    dummy_var ^= ((int64_t) (void*) drop_opaque_MutexDynOverlay);
    dummy_var ^= ((int64_t) (void*) share_opaque_MutexDynOverlay);
    dummy_var ^= ((int64_t) (void*) drop_opaque_MutexOcaAttrRaw);
    dummy_var ^= ((int64_t) (void*) share_opaque_MutexOcaAttrRaw);
    dummy_var ^= ((int64_t) (void*) drop_opaque_MutexOcaBoxRaw);
    dummy_var ^= ((int64_t) (void*) share_opaque_MutexOcaBoxRaw);
    dummy_var ^= ((int64_t) (void*) drop_opaque_MutexOcaBundleRaw);
    dummy_var ^= ((int64_t) (void*) share_opaque_MutexOcaBundleRaw);
    dummy_var ^= ((int64_t) (void*) drop_opaque_MutexOcaCaptureBaseRaw);
    dummy_var ^= ((int64_t) (void*) share_opaque_MutexOcaCaptureBaseRaw);
    dummy_var ^= ((int64_t) (void*) drop_opaque_MutexStringMap);
    dummy_var ^= ((int64_t) (void*) share_opaque_MutexStringMap);
    dummy_var ^= ((int64_t) (void*) free_WireSyncReturn);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    return dummy_var;
}
