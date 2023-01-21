#[doc = "Register `bmx_cfg4` reader"]
pub struct R(crate::R<BMX_CFG4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMX_CFG4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BMX_CFG4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BMX_CFG4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `bmx_cfg4` writer"]
pub struct W(crate::W<BMX_CFG4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BMX_CFG4_SPEC>;
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
impl From<crate::W<BMX_CFG4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BMX_CFG4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sts_bmx_berr_src` reader - "]
pub type STS_BMX_BERR_SRC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `reserved_14_15` reader - "]
pub type RESERVED_14_15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sts_mcu_berr_src` reader - "]
pub type STS_MCU_BERR_SRC_R = crate::BitReader<bool>;
#[doc = "Field `reserved_17_23` reader - "]
pub type RESERVED_17_23_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sts_mcu_berr_id` reader - "]
pub type STS_MCU_BERR_ID_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn sts_bmx_berr_src(&self) -> STS_BMX_BERR_SRC_R {
        STS_BMX_BERR_SRC_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn reserved_14_15(&self) -> RESERVED_14_15_R {
        RESERVED_14_15_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn sts_mcu_berr_src(&self) -> STS_MCU_BERR_SRC_R {
        STS_MCU_BERR_SRC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23"]
    #[inline(always)]
    pub fn reserved_17_23(&self) -> RESERVED_17_23_R {
        RESERVED_17_23_R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn sts_mcu_berr_id(&self) -> STS_MCU_BERR_ID_R {
        STS_MCU_BERR_ID_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "bmx_cfg4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmx_cfg4](index.html) module"]
pub struct BMX_CFG4_SPEC;
impl crate::RegisterSpec for BMX_CFG4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bmx_cfg4::R](R) reader structure"]
impl crate::Readable for BMX_CFG4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bmx_cfg4::W](W) writer structure"]
impl crate::Writable for BMX_CFG4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets bmx_cfg4 to value 0"]
impl crate::Resettable for BMX_CFG4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
