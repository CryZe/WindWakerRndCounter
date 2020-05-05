[OSInit] + 0x118:
; lis r3, 0x8045 Use 804204A0 as ArenaLow
u32 0x3c608042

[main01()] + 0xD0:
b game_loop

[cM_rnd()] + 0xE4:
b count_call

[dSeaFightGame_info_c::init(i32, i32)] + 0x78:
bl init_board
