# build system taken from fractal https://gitlab.gnome.org/World/fractal

project(
  'sentinel', 'rust',
  version: '0.1.0',
  license: 'GPLv3',
)

sentinel_version = meson.project_version()
version_array = sentinel_version.split('.')
sentinel_major_version = version_array[0].to_int()
sentinel_minor_version = version_array[1].to_int()
sentinel_version_micro = version_array[2].to_int()

sentinel_prefix = get_option('prefix')
sentinel_bindir = join_paths(sentinel_prefix, get_option('bindir'))

datadir = get_option('datadir')
icondir = join_paths(datadir, 'icons')
subdir('sentinel-gtk/resources')

cargo = find_program('cargo', required: false)
gresource = find_program('glib-compile-resources', required: false)
cargo_vendor = find_program('cargo-vendor', required: false)
cargo_script = find_program('scripts/cargo.sh')

cargo_release = custom_target('cargo-build',
                        build_by_default: true,
                        build_always: true,
                        output: ['sentinel'],
                        install: true,
                        install_dir: sentinel_bindir,
                        command: [cargo_script, '@CURRENT_SOURCE_DIR@', '@OUTPUT@'])

run_target('release', command: ['scripts/release.sh',
                                meson.project_name() + '-' + sentinel_version
                               ])