{
  "targets": [
    {
      "target_name": "rgblib",
      "sources": [ "swig_wrap.cxx" ],
      "libraries": [
         '-lrgb',
      ],
      "include_dirs": [
         '<(module_root_dir)/include',
       ],
      "ldflags": [],
      "cflags!": ["-std=c++11"],
      'conditions': [
        [
          'OS == "mac"', {
            'libraries': [
              '-L<(module_root_dir)/lib/mac',
            ],
            "ldflags": [
              '-Wl,-rpath,<(module_root_dir)/lib/mac/'
            ]
          },
          'OS == "linux"', {
            'libraries': [
              '-L<(module_root_dir)/lib/linux',
            ],
            "ldflags": [
              '-Wl,-rpath,<(module_root_dir)/lib/linux/'
            ]
          }
        ],
      ]
    }
  ]
}
