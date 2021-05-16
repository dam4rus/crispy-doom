#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Automap Automap;

struct Automap *automap_new(int32_t player_position_x,
                            int32_t player_position_y,
                            int32_t window_width,
                            int32_t window_height,
                            int32_t scale_frame_buffer_to_map);

void automap_free(struct Automap *automap);

void automap_change_window_location(struct Automap *automap,
                                    bool rotate,
                                    int64_t min_x,
                                    int64_t min_y,
                                    int64_t max_x,
                                    int64_t max_y);

void automap_activate_new_scale(struct Automap *automap,
                                int32_t window_width,
                                int32_t window_height,
                                int32_t scale_frame_buffer_to_map);

void automap_update_panning(struct Automap *automap,
                            int64_t pan_increase_keyboard_x,
                            int64_t pan_increase_keyboard_y,
                            int64_t pan_increase_mouse_x,
                            int64_t pan_increase_mouse_y);

void automap_save_rect(struct Automap *automap);

void automap_restore_rect(struct Automap *automap,
                          int32_t player_position_x,
                          int32_t player_position_y);

void automap_follow_player(struct Automap *automap,
                           int32_t player_position_x,
                           int32_t player_position_y);

void automap_print_rect(const struct Automap *automap);

void automap_get_rect(const struct Automap *automap,
                      int64_t *x,
                      int64_t *y,
                      int64_t *width,
                      int64_t *height);
