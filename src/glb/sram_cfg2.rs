#[doc = "Register `sram_cfg2` reader"]
pub struct R(crate::R<SRAM_CFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRAM_CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRAM_CFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRAM_CFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sram_cfg2` writer"]
pub struct W(crate::W<SRAM_CFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRAM_CFG2_SPEC>;
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
impl From<crate::W<SRAM_CFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRAM_CFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_mcu_cache_dvse` reader - "]
pub type CR_MCU_CACHE_DVSE_R = crate::BitReader<bool>;
#[doc = "Field `cr_mcu_cache_dvse` writer - "]
pub type CR_MCU_CACHE_DVSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRAM_CFG2_SPEC, bool, O>;
#[doc = "Field `cr_mcu_hsram_dvse` reader - "]
pub type CR_MCU_HSRAM_DVSE_R = crate::BitReader<bool>;
#[doc = "Field `cr_mcu_hsram_dvse` writer - "]
pub type CR_MCU_HSRAM_DVSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRAM_CFG2_SPEC, bool, O>;
#[doc = "Field `cr_mcu_rom_dvse` reader - "]
pub type CR_MCU_ROM_DVSE_R = crate::BitReader<bool>;
#[doc = "Field `cr_mcu_rom_dvse` writer - "]
pub type CR_MCU_ROM_DVSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRAM_CFG2_SPEC, bool, O>;
#[doc = "Field `cr_wb_ram_dvse` reader - "]
pub type CR_WB_RAM_DVSE_R = crate::BitReader<bool>;
#[doc = "Field `cr_wb_ram_dvse` writer - "]
pub type CR_WB_RAM_DVSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRAM_CFG2_SPEC, bool, O>;
#[doc = "Field `cr_misc_ram_dvse` reader - "]
pub type CR_MISC_RAM_DVSE_R = crate::BitReader<bool>;
#[doc = "Field `cr_misc_ram_dvse` writer - "]
pub type CR_MISC_RAM_DVSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRAM_CFG2_SPEC, bool, O>;
#[doc = "Field `cr_ocram_dvse` reader - "]
pub type CR_OCRAM_DVSE_R = crate::BitReader<bool>;
#[doc = "Field `cr_ocram_dvse` writer - "]
pub type CR_OCRAM_DVSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRAM_CFG2_SPEC, bool, O>;
#[doc = "Field `cr_wram_dvse` reader - "]
pub type CR_WRAM_DVSE_R = crate::BitReader<bool>;
#[doc = "Field `cr_wram_dvse` writer - "]
pub type CR_WRAM_DVSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRAM_CFG2_SPEC, bool, O>;
#[doc = "Field `reserved_7` reader - "]
pub type RESERVED_7_R = crate::BitReader<bool>;
#[doc = "Field `cr_mcu_cache_nap` reader - "]
pub type CR_MCU_CACHE_NAP_R = crate::BitReader<bool>;
#[doc = "Field `cr_mcu_cache_nap` writer - "]
pub type CR_MCU_CACHE_NAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRAM_CFG2_SPEC, bool, O>;
#[doc = "Field `cr_mcu_hsram_nap` reader - "]
pub type CR_MCU_HSRAM_NAP_R = crate::BitReader<bool>;
#[doc = "Field `cr_mcu_hsram_nap` writer - "]
pub type CR_MCU_HSRAM_NAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRAM_CFG2_SPEC, bool, O>;
#[doc = "Field `reserved_10` reader - "]
pub type RESERVED_10_R = crate::BitReader<bool>;
#[doc = "Field `cr_wb_ram_nap` reader - "]
pub type CR_WB_RAM_NAP_R = crate::BitReader<bool>;
#[doc = "Field `cr_wb_ram_nap` writer - "]
pub type CR_WB_RAM_NAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRAM_CFG2_SPEC, bool, O>;
#[doc = "Field `cr_misc_ram_nap` reader - "]
pub type CR_MISC_RAM_NAP_R = crate::BitReader<bool>;
#[doc = "Field `cr_misc_ram_nap` writer - "]
pub type CR_MISC_RAM_NAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRAM_CFG2_SPEC, bool, O>;
#[doc = "Field `cr_ocram_nap` reader - "]
pub type CR_OCRAM_NAP_R = crate::BitReader<bool>;
#[doc = "Field `cr_ocram_nap` writer - "]
pub type CR_OCRAM_NAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRAM_CFG2_SPEC, bool, O>;
#[doc = "Field `cr_wram_nap` reader - "]
pub type CR_WRAM_NAP_R = crate::BitReader<bool>;
#[doc = "Field `cr_wram_nap` writer - "]
pub type CR_WRAM_NAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRAM_CFG2_SPEC, bool, O>;
#[doc = "Field `reserved_15_31` reader - "]
pub type RESERVED_15_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_mcu_cache_dvse(&self) -> CR_MCU_CACHE_DVSE_R {
        CR_MCU_CACHE_DVSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_mcu_hsram_dvse(&self) -> CR_MCU_HSRAM_DVSE_R {
        CR_MCU_HSRAM_DVSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_mcu_rom_dvse(&self) -> CR_MCU_ROM_DVSE_R {
        CR_MCU_ROM_DVSE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_wb_ram_dvse(&self) -> CR_WB_RAM_DVSE_R {
        CR_WB_RAM_DVSE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_misc_ram_dvse(&self) -> CR_MISC_RAM_DVSE_R {
        CR_MISC_RAM_DVSE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_ocram_dvse(&self) -> CR_OCRAM_DVSE_R {
        CR_OCRAM_DVSE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_wram_dvse(&self) -> CR_WRAM_DVSE_R {
        CR_WRAM_DVSE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reserved_7(&self) -> RESERVED_7_R {
        RESERVED_7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_mcu_cache_nap(&self) -> CR_MCU_CACHE_NAP_R {
        CR_MCU_CACHE_NAP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_mcu_hsram_nap(&self) -> CR_MCU_HSRAM_NAP_R {
        CR_MCU_HSRAM_NAP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn reserved_10(&self) -> RESERVED_10_R {
        RESERVED_10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_wb_ram_nap(&self) -> CR_WB_RAM_NAP_R {
        CR_WB_RAM_NAP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_misc_ram_nap(&self) -> CR_MISC_RAM_NAP_R {
        CR_MISC_RAM_NAP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cr_ocram_nap(&self) -> CR_OCRAM_NAP_R {
        CR_OCRAM_NAP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cr_wram_nap(&self) -> CR_WRAM_NAP_R {
        CR_WRAM_NAP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:31"]
    #[inline(always)]
    pub fn reserved_15_31(&self) -> RESERVED_15_31_R {
        RESERVED_15_31_R::new((self.bits >> 15) & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cr_mcu_cache_dvse(&mut self) -> CR_MCU_CACHE_DVSE_W<0> {
        CR_MCU_CACHE_DVSE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cr_mcu_hsram_dvse(&mut self) -> CR_MCU_HSRAM_DVSE_W<1> {
        CR_MCU_HSRAM_DVSE_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn cr_mcu_rom_dvse(&mut self) -> CR_MCU_ROM_DVSE_W<2> {
        CR_MCU_ROM_DVSE_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn cr_wb_ram_dvse(&mut self) -> CR_WB_RAM_DVSE_W<3> {
        CR_WB_RAM_DVSE_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn cr_misc_ram_dvse(&mut self) -> CR_MISC_RAM_DVSE_W<4> {
        CR_MISC_RAM_DVSE_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn cr_ocram_dvse(&mut self) -> CR_OCRAM_DVSE_W<5> {
        CR_OCRAM_DVSE_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn cr_wram_dvse(&mut self) -> CR_WRAM_DVSE_W<6> {
        CR_WRAM_DVSE_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn cr_mcu_cache_nap(&mut self) -> CR_MCU_CACHE_NAP_W<8> {
        CR_MCU_CACHE_NAP_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn cr_mcu_hsram_nap(&mut self) -> CR_MCU_HSRAM_NAP_W<9> {
        CR_MCU_HSRAM_NAP_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn cr_wb_ram_nap(&mut self) -> CR_WB_RAM_NAP_W<11> {
        CR_WB_RAM_NAP_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn cr_misc_ram_nap(&mut self) -> CR_MISC_RAM_NAP_W<12> {
        CR_MISC_RAM_NAP_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn cr_ocram_nap(&mut self) -> CR_OCRAM_NAP_W<13> {
        CR_OCRAM_NAP_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn cr_wram_nap(&mut self) -> CR_WRAM_NAP_W<14> {
        CR_WRAM_NAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "reg_sram_parm\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sram_cfg2](index.html) module"]
pub struct SRAM_CFG2_SPEC;
impl crate::RegisterSpec for SRAM_CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sram_cfg2::R](R) reader structure"]
impl crate::Readable for SRAM_CFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sram_cfg2::W](W) writer structure"]
impl crate::Writable for SRAM_CFG2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sram_cfg2 to value 0"]
impl crate::Resettable for SRAM_CFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
