#![no_std]

use header::{Tag, TagIter};
pub use boot_loader_name::BootLoaderNameTag;
pub use elf_sections::{ElfSectionsTag, ElfSection, ElfSectionIter, ElfSectionType, ElfSectionFlags, StringTable};
pub use elf_sections::{ELF_SECTION_WRITABLE, ELF_SECTION_ALLOCATED, ELF_SECTION_EXECUTABLE};
pub use memory_map::{MemoryMapTag, MemoryArea, MemoryAreaIter};
pub use module::{ModuleTag, ModuleIter};

#[macro_use]
extern crate bitflags;

mod header;
mod boot_loader_name;
mod elf_sections;
mod memory_map;
mod module;

pub unsafe fn load(address: usize) -> &'static BootInformation {
    let multiboot = &*(address as *const BootInformation);
    assert!(multiboot.has_valid_end_tag());
    multiboot
}

#[repr(C)]
pub struct BootInformation {
    pub total_size: u32,
    _reserved: u32,
    first_tag: Tag,
}

impl BootInformation {
    pub fn start_address(&self) -> usize {
        self as *const _ as usize
    }

    pub fn end_address(&self) -> usize {
        self.start_address() + self.total_size as usize
    }

    pub fn elf_sections_tag(&self) -> Option<&'static ElfSectionsTag> {
        self.get_tag(9).map(|tag| unsafe{&*(tag as *const Tag as *const ElfSectionsTag)})
    }

    pub fn memory_map_tag(&self) -> Option<&'static MemoryMapTag> {
        self.get_tag(6).map(|tag| unsafe{&*(tag as *const Tag as *const MemoryMapTag)})
    }

    pub fn module_tags(&self) -> ModuleIter {
        ModuleIter{ iter: self.tags() }
    }

    pub fn boot_loader_name_tag(&self) -> Option<&'static BootLoaderNameTag> {
        self.get_tag(2).map(|tag| unsafe{&*(tag as *const Tag as *const BootLoaderNameTag)})
    }

    fn has_valid_end_tag(&self) -> bool {
        const END_TAG: Tag = Tag{typ:0, size:8};

        let self_ptr = self as *const _;
        let end_tag_addr = self_ptr as usize + (self.total_size - END_TAG.size) as usize;
        let end_tag = unsafe{&*(end_tag_addr as *const Tag)};

        end_tag.typ == END_TAG.typ && end_tag.size == END_TAG.size
    }

    fn get_tag(&self, typ: u32) -> Option<&'static Tag> {
        self.tags().find(|tag| tag.typ == typ)
    }

    fn tags(&self) -> TagIter {
        TagIter{current: &self.first_tag as *const _}
    }
}
