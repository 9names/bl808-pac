#[doc = "Register `bmx_cfg3` reader"]
pub struct R(crate::R<BMX_CFG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMX_CFG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BMX_CFG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BMX_CFG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `bmx_cfg3` writer"]
pub struct W(crate::W<BMX_CFG3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BMX_CFG3_SPEC>;
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
impl From<crate::W<BMX_CFG3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BMX_CFG3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_bmx_berr_clr` reader - "]
pub type REG_BMX_BERR_CLR_R = crate::BitReader<bool>;
#[doc = "Field `reg_bmx_berr_clr` writer - "]
pub type REG_BMX_BERR_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMX_CFG3_SPEC, bool, O>;
#[doc = "Field `reg_bmx_berr_last` reader - "]
pub type REG_BMX_BERR_LAST_R = crate::BitReader<bool>;
#[doc = "Field `reg_bmx_berr_last` writer - "]
pub type REG_BMX_BERR_LAST_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMX_CFG3_SPEC, bool, O>;
#[doc = "Field `reserved_2_7` reader - "]
pub type RESERVED_2_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_mcu_berr_clr` reader - "]
pub type REG_MCU_BERR_CLR_R = crate::BitReader<bool>;
#[doc = "Field `reg_mcu_berr_clr` writer - "]
pub type REG_MCU_BERR_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMX_CFG3_SPEC, bool, O>;
#[doc = "Field `reg_mcu_berr_last` reader - "]
pub type REG_MCU_BERR_LAST_R = crate::BitReader<bool>;
#[doc = "Field `reg_mcu_berr_last` writer - "]
pub type REG_MCU_BERR_LAST_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMX_CFG3_SPEC, bool, O>;
#[doc = "Field `reserved_10_15` reader - "]
pub type RESERVED_10_15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sts_bmx_berr` reader - "]
pub type STS_BMX_BERR_R = crate::BitReader<bool>;
#[doc = "Field `sts_mcu_berr` reader - "]
pub type STS_MCU_BERR_R = crate::BitReader<bool>;
#[doc = "Field `reserved_18_23` reader - "]
pub type RESERVED_18_23_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sts_bmx_berr_write` reader - "]
pub type STS_BMX_BERR_WRITE_R = crate::BitReader<bool>;
#[doc = "Field `sts_mcu_berr_write` reader - "]
pub type STS_MCU_BERR_WRITE_R = crate::BitReader<bool>;
#[doc = "Field `reserved_26_31` reader - "]
pub type RESERVED_26_31_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_bmx_berr_clr(&self) -> REG_BMX_BERR_CLR_R {
        REG_BMX_BERR_CLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_bmx_berr_last(&self) -> REG_BMX_BERR_LAST_R {
        REG_BMX_BERR_LAST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7"]
    #[inline(always)]
    pub fn reserved_2_7(&self) -> RESERVED_2_7_R {
        RESERVED_2_7_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_mcu_berr_clr(&self) -> REG_MCU_BERR_CLR_R {
        REG_MCU_BERR_CLR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg_mcu_berr_last(&self) -> REG_MCU_BERR_LAST_R {
        REG_MCU_BERR_LAST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15"]
    #[inline(always)]
    pub fn reserved_10_15(&self) -> RESERVED_10_15_R {
        RESERVED_10_15_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn sts_bmx_berr(&self) -> STS_BMX_BERR_R {
        STS_BMX_BERR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn sts_mcu_berr(&self) -> STS_MCU_BERR_R {
        STS_MCU_BERR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:23"]
    #[inline(always)]
    pub fn reserved_18_23(&self) -> RESERVED_18_23_R {
        RESERVED_18_23_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn sts_bmx_berr_write(&self) -> STS_BMX_BERR_WRITE_R {
        STS_BMX_BERR_WRITE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn sts_mcu_berr_write(&self) -> STS_MCU_BERR_WRITE_R {
        STS_MCU_BERR_WRITE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:31"]
    #[inline(always)]
    pub fn reserved_26_31(&self) -> RESERVED_26_31_R {
        RESERVED_26_31_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bmx_berr_clr(&mut self) -> REG_BMX_BERR_CLR_W<0> {
        REG_BMX_BERR_CLR_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bmx_berr_last(&mut self) -> REG_BMX_BERR_LAST_W<1> {
        REG_BMX_BERR_LAST_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn reg_mcu_berr_clr(&mut self) -> REG_MCU_BERR_CLR_W<8> {
        REG_MCU_BERR_CLR_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn reg_mcu_berr_last(&mut self) -> REG_MCU_BERR_LAST_W<9> {
        REG_MCU_BERR_LAST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "bmx_cfg3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmx_cfg3](index.html) module"]
pub struct BMX_CFG3_SPEC;
impl crate::RegisterSpec for BMX_CFG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bmx_cfg3::R](R) reader structure"]
impl crate::Readable for BMX_CFG3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bmx_cfg3::W](W) writer structure"]
impl crate::Writable for BMX_CFG3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets bmx_cfg3 to value 0"]
impl crate::Resettable for BMX_CFG3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
