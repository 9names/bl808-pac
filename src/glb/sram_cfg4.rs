#[doc = "Register `sram_cfg4` reader"]
pub struct R(crate::R<SRAM_CFG4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRAM_CFG4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRAM_CFG4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRAM_CFG4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sram_cfg4` writer"]
pub struct W(crate::W<SRAM_CFG4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRAM_CFG4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SRAM_CFG4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRAM_CFG4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_mcu_cache_dvs` reader - "]
pub type CR_MCU_CACHE_DVS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_mcu_cache_dvs` writer - "]
pub type CR_MCU_CACHE_DVS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SRAM_CFG4_SPEC, u8, u8, 4, O>;
#[doc = "Field `cr_mcu_hsram_dvs` reader - "]
pub type CR_MCU_HSRAM_DVS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_mcu_hsram_dvs` writer - "]
pub type CR_MCU_HSRAM_DVS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SRAM_CFG4_SPEC, u8, u8, 4, O>;
#[doc = "Field `cr_mcu_rom_dvs` reader - "]
pub type CR_MCU_ROM_DVS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_mcu_rom_dvs` writer - "]
pub type CR_MCU_ROM_DVS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SRAM_CFG4_SPEC, u8, u8, 4, O>;
#[doc = "Field `cr_wb_ram_dvs` reader - "]
pub type CR_WB_RAM_DVS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_wb_ram_dvs` writer - "]
pub type CR_WB_RAM_DVS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SRAM_CFG4_SPEC, u8, u8, 4, O>;
#[doc = "Field `cr_misc_ram_dvs` reader - "]
pub type CR_MISC_RAM_DVS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_misc_ram_dvs` writer - "]
pub type CR_MISC_RAM_DVS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SRAM_CFG4_SPEC, u8, u8, 4, O>;
#[doc = "Field `cr_ocram_dvs` reader - "]
pub type CR_OCRAM_DVS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_ocram_dvs` writer - "]
pub type CR_OCRAM_DVS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SRAM_CFG4_SPEC, u8, u8, 4, O>;
#[doc = "Field `cr_wram_dvs` reader - "]
pub type CR_WRAM_DVS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_wram_dvs` writer - "]
pub type CR_WRAM_DVS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SRAM_CFG4_SPEC, u8, u8, 4, O>;
#[doc = "Field `reserved_28_31` reader - "]
pub type RESERVED_28_31_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn cr_mcu_cache_dvs(&self) -> CR_MCU_CACHE_DVS_R {
        CR_MCU_CACHE_DVS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn cr_mcu_hsram_dvs(&self) -> CR_MCU_HSRAM_DVS_R {
        CR_MCU_HSRAM_DVS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn cr_mcu_rom_dvs(&self) -> CR_MCU_ROM_DVS_R {
        CR_MCU_ROM_DVS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn cr_wb_ram_dvs(&self) -> CR_WB_RAM_DVS_R {
        CR_WB_RAM_DVS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn cr_misc_ram_dvs(&self) -> CR_MISC_RAM_DVS_R {
        CR_MISC_RAM_DVS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn cr_ocram_dvs(&self) -> CR_OCRAM_DVS_R {
        CR_OCRAM_DVS_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn cr_wram_dvs(&self) -> CR_WRAM_DVS_R {
        CR_WRAM_DVS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn reserved_28_31(&self) -> RESERVED_28_31_R {
        RESERVED_28_31_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn cr_mcu_cache_dvs(&mut self) -> CR_MCU_CACHE_DVS_W<0> {
        CR_MCU_CACHE_DVS_W::new(self)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn cr_mcu_hsram_dvs(&mut self) -> CR_MCU_HSRAM_DVS_W<4> {
        CR_MCU_HSRAM_DVS_W::new(self)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn cr_mcu_rom_dvs(&mut self) -> CR_MCU_ROM_DVS_W<8> {
        CR_MCU_ROM_DVS_W::new(self)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    #[must_use]
    pub fn cr_wb_ram_dvs(&mut self) -> CR_WB_RAM_DVS_W<12> {
        CR_WB_RAM_DVS_W::new(self)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn cr_misc_ram_dvs(&mut self) -> CR_MISC_RAM_DVS_W<16> {
        CR_MISC_RAM_DVS_W::new(self)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    #[must_use]
    pub fn cr_ocram_dvs(&mut self) -> CR_OCRAM_DVS_W<20> {
        CR_OCRAM_DVS_W::new(self)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    #[must_use]
    pub fn cr_wram_dvs(&mut self) -> CR_WRAM_DVS_W<24> {
        CR_WRAM_DVS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "reg_sram_parm2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sram_cfg4](index.html) module"]
pub struct SRAM_CFG4_SPEC;
impl crate::RegisterSpec for SRAM_CFG4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sram_cfg4::R](R) reader structure"]
impl crate::Readable for SRAM_CFG4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sram_cfg4::W](W) writer structure"]
impl crate::Writable for SRAM_CFG4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sram_cfg4 to value 0x0ccc_cccc"]
impl crate::Resettable for SRAM_CFG4_SPEC {
    const RESET_VALUE: Self::Ux = 0x0ccc_cccc;
}
