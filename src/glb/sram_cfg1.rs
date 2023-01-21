#[doc = "Register `sram_cfg1` reader"]
pub struct R(crate::R<SRAM_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRAM_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRAM_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRAM_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sram_cfg1` writer"]
pub struct W(crate::W<SRAM_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRAM_CFG1_SPEC>;
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
impl From<crate::W<SRAM_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRAM_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_mcu_cache_slp` reader - "]
pub type CR_MCU_CACHE_SLP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_mcu_cache_slp` writer - "]
pub type CR_MCU_CACHE_SLP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SRAM_CFG1_SPEC, u8, u8, 2, O>;
#[doc = "Field `cr_mcu_hsram_slp` reader - "]
pub type CR_MCU_HSRAM_SLP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_mcu_hsram_slp` writer - "]
pub type CR_MCU_HSRAM_SLP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SRAM_CFG1_SPEC, u8, u8, 4, O>;
#[doc = "Field `cr_mcu_rom_slp` reader - "]
pub type CR_MCU_ROM_SLP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_mcu_rom_slp` writer - "]
pub type CR_MCU_ROM_SLP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SRAM_CFG1_SPEC, u8, u8, 2, O>;
#[doc = "Field `cr_wb_ram_slp` reader - "]
pub type CR_WB_RAM_SLP_R = crate::BitReader<bool>;
#[doc = "Field `cr_wb_ram_slp` writer - "]
pub type CR_WB_RAM_SLP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRAM_CFG1_SPEC, bool, O>;
#[doc = "Field `cr_misc_ram_slp` reader - "]
pub type CR_MISC_RAM_SLP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_misc_ram_slp` writer - "]
pub type CR_MISC_RAM_SLP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SRAM_CFG1_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_11_31` reader - "]
pub type RESERVED_11_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn cr_mcu_cache_slp(&self) -> CR_MCU_CACHE_SLP_R {
        CR_MCU_CACHE_SLP_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn cr_mcu_hsram_slp(&self) -> CR_MCU_HSRAM_SLP_R {
        CR_MCU_HSRAM_SLP_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn cr_mcu_rom_slp(&self) -> CR_MCU_ROM_SLP_R {
        CR_MCU_ROM_SLP_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_wb_ram_slp(&self) -> CR_WB_RAM_SLP_R {
        CR_WB_RAM_SLP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn cr_misc_ram_slp(&self) -> CR_MISC_RAM_SLP_R {
        CR_MISC_RAM_SLP_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:31"]
    #[inline(always)]
    pub fn reserved_11_31(&self) -> RESERVED_11_31_R {
        RESERVED_11_31_R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn cr_mcu_cache_slp(&mut self) -> CR_MCU_CACHE_SLP_W<0> {
        CR_MCU_CACHE_SLP_W::new(self)
    }
    #[doc = "Bits 2:5"]
    #[inline(always)]
    #[must_use]
    pub fn cr_mcu_hsram_slp(&mut self) -> CR_MCU_HSRAM_SLP_W<2> {
        CR_MCU_HSRAM_SLP_W::new(self)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn cr_mcu_rom_slp(&mut self) -> CR_MCU_ROM_SLP_W<6> {
        CR_MCU_ROM_SLP_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn cr_wb_ram_slp(&mut self) -> CR_WB_RAM_SLP_W<8> {
        CR_WB_RAM_SLP_W::new(self)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    #[must_use]
    pub fn cr_misc_ram_slp(&mut self) -> CR_MISC_RAM_SLP_W<9> {
        CR_MISC_RAM_SLP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "reg_sram_slp\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sram_cfg1](index.html) module"]
pub struct SRAM_CFG1_SPEC;
impl crate::RegisterSpec for SRAM_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sram_cfg1::R](R) reader structure"]
impl crate::Readable for SRAM_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sram_cfg1::W](W) writer structure"]
impl crate::Writable for SRAM_CFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sram_cfg1 to value 0"]
impl crate::Resettable for SRAM_CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
