#[doc = "Register `tzc_ocram_tzsrg_ctrl` reader"]
pub struct R(crate::R<TZC_OCRAM_TZSRG_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_OCRAM_TZSRG_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_OCRAM_TZSRG_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_OCRAM_TZSRG_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tzc_ocram_tzsrg_ctrl` writer"]
pub struct W(crate::W<TZC_OCRAM_TZSRG_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_OCRAM_TZSRG_CTRL_SPEC>;
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
impl From<crate::W<TZC_OCRAM_TZSRG_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_OCRAM_TZSRG_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tzc_ocram_tzsrg_r0_id_en` reader - "]
pub type TZC_OCRAM_TZSRG_R0_ID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_ocram_tzsrg_r1_id_en` reader - "]
pub type TZC_OCRAM_TZSRG_R1_ID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_ocram_tzsrg_r2_id_en` reader - "]
pub type TZC_OCRAM_TZSRG_R2_ID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_ocram_tzsrg_rx_id_en` reader - "]
pub type TZC_OCRAM_TZSRG_RX_ID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_ocram_tzsrg_r0_en` reader - "]
pub type TZC_OCRAM_TZSRG_R0_EN_R = crate::BitReader<bool>;
#[doc = "Field `tzc_ocram_tzsrg_r1_en` reader - "]
pub type TZC_OCRAM_TZSRG_R1_EN_R = crate::BitReader<bool>;
#[doc = "Field `tzc_ocram_tzsrg_r2_en` reader - "]
pub type TZC_OCRAM_TZSRG_R2_EN_R = crate::BitReader<bool>;
#[doc = "Field `tzc_ocram_tzsrg_rx_en` reader - "]
pub type TZC_OCRAM_TZSRG_RX_EN_R = crate::BitReader<bool>;
#[doc = "Field `tzc_ocram_tzsrg_r0_lock` reader - "]
pub type TZC_OCRAM_TZSRG_R0_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_ocram_tzsrg_r1_lock` reader - "]
pub type TZC_OCRAM_TZSRG_R1_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_ocram_tzsrg_r2_lock` reader - "]
pub type TZC_OCRAM_TZSRG_R2_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_ocram_tzsrg_rx_lock` reader - "]
pub type TZC_OCRAM_TZSRG_RX_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `reserved_24_31` reader - "]
pub type RESERVED_24_31_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r0_id_en(&self) -> TZC_OCRAM_TZSRG_R0_ID_EN_R {
        TZC_OCRAM_TZSRG_R0_ID_EN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r1_id_en(&self) -> TZC_OCRAM_TZSRG_R1_ID_EN_R {
        TZC_OCRAM_TZSRG_R1_ID_EN_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r2_id_en(&self) -> TZC_OCRAM_TZSRG_R2_ID_EN_R {
        TZC_OCRAM_TZSRG_R2_ID_EN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_rx_id_en(&self) -> TZC_OCRAM_TZSRG_RX_ID_EN_R {
        TZC_OCRAM_TZSRG_RX_ID_EN_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r0_en(&self) -> TZC_OCRAM_TZSRG_R0_EN_R {
        TZC_OCRAM_TZSRG_R0_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r1_en(&self) -> TZC_OCRAM_TZSRG_R1_EN_R {
        TZC_OCRAM_TZSRG_R1_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r2_en(&self) -> TZC_OCRAM_TZSRG_R2_EN_R {
        TZC_OCRAM_TZSRG_R2_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_rx_en(&self) -> TZC_OCRAM_TZSRG_RX_EN_R {
        TZC_OCRAM_TZSRG_RX_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r0_lock(&self) -> TZC_OCRAM_TZSRG_R0_LOCK_R {
        TZC_OCRAM_TZSRG_R0_LOCK_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r1_lock(&self) -> TZC_OCRAM_TZSRG_R1_LOCK_R {
        TZC_OCRAM_TZSRG_R1_LOCK_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r2_lock(&self) -> TZC_OCRAM_TZSRG_R2_LOCK_R {
        TZC_OCRAM_TZSRG_R2_LOCK_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_rx_lock(&self) -> TZC_OCRAM_TZSRG_RX_LOCK_R {
        TZC_OCRAM_TZSRG_RX_LOCK_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn reserved_24_31(&self) -> RESERVED_24_31_R {
        RESERVED_24_31_R::new(((self.bits >> 24) & 0xff) as u8)
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
#[doc = "tzc_ocram_tzsrg_ctrl\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_ocram_tzsrg_ctrl](index.html) module"]
pub struct TZC_OCRAM_TZSRG_CTRL_SPEC;
impl crate::RegisterSpec for TZC_OCRAM_TZSRG_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_ocram_tzsrg_ctrl::R](R) reader structure"]
impl crate::Readable for TZC_OCRAM_TZSRG_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_ocram_tzsrg_ctrl::W](W) writer structure"]
impl crate::Writable for TZC_OCRAM_TZSRG_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_ocram_tzsrg_ctrl to value 0xffff"]
impl crate::Resettable for TZC_OCRAM_TZSRG_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
