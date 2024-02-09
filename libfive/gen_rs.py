import os
from datetime import datetime
from itertools import repeat

import parse

################################################################################
def arg_in(a, use_self):
    if a.type == 'libfive_tree':
        if use_self and a.index == 0:
            return 'self'
        else:
            return '{}: Tree'.format(a.name)
    elif a.type == 'tfloat':
        return '{}: TreeFloat'.format(a.name)
    elif a.type == 'tvec2':
        return '{}: TreeVec2'.format(a.name)
    elif a.type == 'tvec3':
        return '{}: TreeVec3'.format(a.name)
    elif a.type == 'int':
        return '{}: u32'.format(a.name)
    elif a.type == 'const char*':
        return '{}: &CStr'.format(a.name)
    else:
        raise RuntimeError("Unknown type %s" % a.type)

def arg_out(a, use_self):
    if a.type == 'libfive_tree':
        if use_self and a.index == 0:
            return 'self.0'
        else:
            return '{}.0'.format(a.name)
    elif a.type == 'tfloat':
        return '{}.0'.format(a.name)
    elif a.type == 'tvec2':
        return 'sys::tvec2 {{ x: {0}.x.0, y: {0}.y.0 }}'.format(a.name)
    elif a.type == 'tvec3':
        return 'sys::tvec3 {{ x: {0}.x.0, y: {0}.y.0, z: {0}.z.0 }}'.format(a.name)
    elif a.type == 'int':
        return '{}.try_into().unwrap()'.format(a.name)
    elif a.type == 'const char*':
        return '{}.as_ptr()'.format(a.name)
    else:
        raise RuntimeError("Unknown type %s" % a.type)

def format_module_modifier(lib, m):
    out = '''
/// # {} <a name="{}"></a>
impl Tree {{
'''.format(m.title(), m)

    for f in lib[m].shapes:
        args_in = ", ".join(map(arg_in, f.args, repeat(True)))
        args_out = ", ".join(map(arg_out, f.args, repeat(True)))
        out += '''
    pub fn {name}({args_in}) -> Self {{
        Self(unsafe {{ sys::{raw_name}{u}({args_out}) }})
    }}
'''.format(name='moveit' if f.name.endswith('move') else f.name,
           raw_name=f.raw_name,
           u='',
           args_in=args_in,
           args_out=args_out)

    out += '}'
    return out

def format_module_generator(lib, m):
    out = '''
/// # {} <a name="{}"></a>
impl Tree {{
'''.format(m.title(), m)

    for f in lib[m].shapes:
        args_in = ", ".join(map(arg_in, f.args, repeat(False)))
        args_out = ", ".join(map(arg_out, f.args, repeat(False)))
        out += '''
    pub fn {name}({args_in}) -> Self {{
        Self(unsafe {{ sys::{raw_name}{u}({args_out}) }})
    }}
'''.format(name=f.name,
           raw_name=f.raw_name,
           u='',
           args_in=args_in,
           args_out=args_out)

    out += '}'
    return out

################################################################################

def arg_call(a):
    if a.type == 'libfive_tree':
        return 'libfive::Tree'
    elif a.type == 'tfloat':
        return 'TreeFloat' # same as float
    elif a.type == 'tvec2':
        return 'TreeVec2'
    elif a.type == 'tvec3':
        return 'TreeVec3'
    elif a.type in ['float', 'int', 'const char*']:
        return a.type
    else:
        raise RuntimeError("Unknown arg type {}".format(a.type))


################################################################################

stdlib = parse.parse_stdlib()

def write_header(f, m):
    f.write(
'''//
// Rust API for the libfive standard library’s ‘{}’ module.
//
// DO NOT EDIT BY HAND!
// This file is automatically generated from libfive/stdlib/stdlib.h.
//
// It was last generated on {} by user {}.
//
'''.format(m, datetime.now().strftime("%Y-%m-%d %H:%M:%S"), os.getlogin()))

stdlib = parse.parse_stdlib()
for m in ['csg', 'transforms']:
    with open('../bind/rust/libfive/stdlib/%s.rs' % m, 'w') as f:
        write_header(f, m)
        f.write(format_module_modifier(stdlib, m))

for m in ['shapes', 'generators']:
     with open('../bind/rust/libfive/stdlib/%s.rs' % m, 'w') as f:
        write_header(f, m)
        f.write(format_module_generator(stdlib, m))

f = open('../bind/rust/libfive/stdlib/text.rs', 'w')
f.write(
'''
/// # Text <a name="text"></a>
impl Tree {
    pub fn text(txt: impl Into<Vec<u8>>, pos: TreeVec2) -> Self {
        let txt = std::ffi::CString::new(txt).unwrap();
        Self(unsafe { sys::text(txt.as_ptr(), sys::tvec2 { x: pos.x.0, y: pos.y.0 }) })
    }
}
'''
)
